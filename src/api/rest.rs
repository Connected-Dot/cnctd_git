use cnctd_rest::Rest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::anyhow;

use crate::{GitProvider, account::GitAccount, repo::{GitRepo, GitRelease}};

#[derive(Debug, Serialize, Deserialize)]
struct GitHubCreateRepoPayload {
    name: String,
    description: String,
    private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubCreateReleasePayload {
    tag_name: String,
    target_commitish: String,
    name: String,
    body: String,
    draft: bool,
    prerelease: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GitRest {
    base_url: String,
}

impl GitRest {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.github.com".to_string()
        }
    }

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

    pub async fn create_repo(git_account: &GitAccount, repo_name: &str, description: &str, is_private: bool) -> anyhow::Result<GitRepo> {
        match git_account.provider {
            GitProvider::GitHub => {
                let base_url = Self::new().base_url;
                let is_org = git_account.url_is_org();

                let url = match is_org {
                    true => {
                        format!(
                            "{}/orgs/{}/repos", 
                            base_url, 
                            git_account.get_org_from_url()
                        )
                    }
                    false => {
                        format!(
                            "{}/user/repos", 
                            base_url
                        )
                    }
                };
                let payload = GitHubCreateRepoPayload {
                    name: repo_name.to_string(),
                    description: description.to_string(),
                    private: is_private,
                };

                let res: GitRepo = Rest::post_with_auth(
                    &url,
                    &git_account.token,
                    payload
                ).await?;

                Ok(res)
            },
            // Add logic for other providers here...
            _ => Err(anyhow!("Provider not yet supported")),
        }
    }

    pub async fn get_repo(git_account: &GitAccount, repo_name: &str) -> anyhow::Result<GitRepo> {
        let git_rest = Self::new();
        let url = match git_account.url_is_org() {
            true => {
                format!(
                    "{}/repos/{}/{}", 
                    git_rest.base_url, 
                    git_account.get_org_from_url(), 
                    repo_name
                )
            }
            false => {
                format!(
                    "{}/repos/{}/{}", 
                    git_rest.base_url, 
                    git_account.login, 
                    repo_name
                )
            }
        };
        println!("url: {}", url);
        let res: GitRepo = Rest::get_with_auth(&url, &git_account.token).await?;

        Ok(res)
    }
    
    pub async fn create_release(
        git_account: &GitAccount,
        repo_name: &str,
        tag_name: &str,
        target_commitish: &str,
        name: &str,
        body: &str,
        draft: bool,
        prerelease: bool,
    ) -> anyhow::Result<GitRelease> {
        match git_account.provider {
            GitProvider::GitHub => {
                let base_url = Self::new().base_url;
                let is_org = git_account.url_is_org();

                let url = match is_org {
                    true => {
                        format!(
                            "{}/repos/{}/{}/releases",
                            base_url,
                            git_account.get_org_from_url(),
                            repo_name
                        )
                    }
                    false => {
                        format!(
                            "{}/repos/{}/{}/releases",
                            base_url,
                            git_account.login,
                            repo_name
                        )
                    }
                };

                let payload = GitHubCreateReleasePayload {
                    tag_name: tag_name.to_string(),
                    target_commitish: target_commitish.to_string(),
                    name: name.to_string(),
                    body: body.to_string(),
                    draft,
                    prerelease,
                };

                let res = Rest::post_with_auth(&url, &git_account.token, payload).await?;

                Ok(res)
            }
            // Add logic for other providers here...
            _ => Err(anyhow!("Provider not yet supported")),
        }
    }

    pub async fn get_latest_release(git_account: &GitAccount, repo_name: &str) -> anyhow::Result<GitRelease> {
        let git_rest = Self::new();
        let url = match git_account.url_is_org() {
            true => {
                format!(
                    "{}/repos/{}/{}/releases/latest", 
                    git_rest.base_url, 
                    git_account.get_org_from_url(), 
                    repo_name
                )
            }
            false => {
                format!(
                    "{}/repos/{}/{}/releases/latest", 
                    git_rest.base_url, 
                    git_account.login, 
                    repo_name
                )
            }
        };
        println!("url: {}", url);
        let res: GitRelease = Rest::get_with_auth(&url, &git_account.token).await?;

        Ok(res)
    }
}