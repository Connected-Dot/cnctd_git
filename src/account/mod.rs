use std::fmt::{Display, self};

use cnctd_rest::Rest;
use serde::{Deserialize, Serialize};
use anyhow::anyhow;

use crate::{GitProvider, api::rest::GitRest};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitAccount {
    pub provider: GitProvider,
    pub login: String,
    pub token: String,
    pub personal_url: String,
    pub org_urls: Vec<String>,
    pub default_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitUser {
    pub login: String,
}

impl GitUser {
    pub async fn get(provider: &GitProvider, token: &str) -> anyhow::Result<Self> {
        match provider {
            GitProvider::GitHub => {
                let user: Self = Rest::get_with_auth("https://api.github.com/user", token).await?;
                Ok(user)
            }
            _ => {
                Err(anyhow!("Provider not supported yet"))
            }
        }   
    }
}

impl Display for GitAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = &self.login;
        write!(f, "{}", display_str)
    }
}
impl GitAccount {
    pub async fn new(provider: GitProvider, token: &str) -> anyhow::Result<Self> {
        let login = GitUser::get(&provider, token).await?.login;

        let personal_url = match provider {
            GitProvider::GitHub => format!("https://github.com/{}", login),
            GitProvider::GitLab => format!("https://gitlab.com/{}", login),
            GitProvider::Bitbucket => format!("https://bitbucket.org/{}", login),
        };
        
        // Fetch organizations based on the provider
        let org_urls = GitRest::org_urls(&provider, token).await?;


        Ok(Self {
            provider,
            login: login.to_string(),
            token: token.to_string(),
            personal_url: personal_url.clone(),
            org_urls,
            default_url: personal_url,
        })
    }

    pub fn url_is_org(&self) -> bool {
        self.org_urls.contains(&self.default_url)
    }

    pub fn get_org_from_url(&self) -> String {
        let parts: Vec<&str> = self.default_url.split('/').collect();
        parts.last().map(|s| s.to_string()).unwrap()
    }

}