use cnctd_rest::Rest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::anyhow;

use crate::{GitProvider, account::GitAccount};

#[derive(Debug, Serialize)]
struct GitHubCreateRepoPayload {
    name: String,
    description: Option<String>,
    private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRest {
    
}

impl GitRest {
    pub async fn org_urls(provider: &GitProvider, token: &str) -> anyhow::Result<Vec<String>> {
        match &provider {
            GitProvider::GitHub => {
                let orgs: Vec<Value> = Rest::get_with_auth("https://api.github.com/user/orgs", token).await?;
                let urls: Vec<String> = orgs.iter()
                    .map(|org| format!("https://github.com/{}", org["login"].as_str().unwrap()))
                    .collect();
                
                Ok(urls)
            },
            // Add logic for other providers here...
            _ => Err(anyhow!("provider not yet supported")),
        }
    }

    pub async fn create_repo(git_account: &GitAccount, repo_name: &str, description: Option<&str>, is_private: bool) -> anyhow::Result<()> {
        match git_account.provider {
            GitProvider::GitHub => {
                let payload = GitHubCreateRepoPayload {
                    name: repo_name.to_string(),
                    description: description.map(String::from),
                    private: is_private,
                };

                let res: serde_json::Value = Rest::post_with_auth(
                    "https://api.github.com/user/repos",
                    &git_account.token,
                    payload
                ).await?;

                if res["id"].is_number() {  // Assuming the 'id' field is present in a successful response
                    Ok(())
                } else {
                    let err_msg = format!("Failed to create repo: {:?}", res);
                    Err(anyhow!(err_msg))
                }
            },
            // Add logic for other providers here...
            _ => Err(anyhow!("Provider not yet supported")),
        }
    }
}