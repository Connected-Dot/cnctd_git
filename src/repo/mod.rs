use std::{fs::File, io::Write, path::{PathBuf, Path}};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
pub use gitignores::Root as ProjectType;

use crate::{api::{rest::GitRest, local::{GitLocal, Branch}}, account::GitAccount, from_str_to_naive_datetime};
#[derive(Debug, Serialize, Deserialize)]
pub struct GitRelease {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    #[serde(deserialize_with = "from_str_to_naive_datetime")]
    pub created_at: NaiveDateTime,
    #[serde(deserialize_with = "from_str_to_naive_datetime")]
    pub published_at: NaiveDateTime,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepo {
    pub id: usize,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub description: Option<String>,
    pub url: String,
    pub html_url: String,
    #[serde(deserialize_with = "from_str_to_naive_datetime")]
    pub created_at: NaiveDateTime,
    #[serde(deserialize_with = "from_str_to_naive_datetime")]
    pub updated_at: NaiveDateTime,
}

impl GitRepo {
    pub async fn new(git_account: &GitAccount, repo_name: &str, description: Option<&str>, is_private: bool) -> anyhow::Result<Self> {
        let repo = GitRest::create_repo(git_account, repo_name, description.unwrap_or(""), is_private).await?;

        Ok(repo)
    }

    pub async fn  get(git_account: &GitAccount, repo_name: &str) -> anyhow::Result<Self> {
        let repo = GitRest::get_repo(git_account, repo_name).await?;

        Ok(repo)

    }

    pub fn init(path: &str) -> anyhow::Result<()> {
        GitLocal::init(path)?;
        File::create(format!("{}/README.md", path))?;
        
        Ok(())
    }

    pub async fn new_release(
        git_account: &GitAccount, 
        repo_name: &str, 
        name: &str, 
        tag_name: &str, 
        target_commitish: &str, 
        body: &str, 
        draft: bool, 
        prerelease: bool
    ) -> anyhow::Result<GitRelease> {
        let release = GitRest::create_release(git_account, repo_name, tag_name, target_commitish, name, body, draft, prerelease).await?;

        Ok(release)
    }

    pub async fn get_latest_release(git_account: &GitAccount, repo_name: &str) -> anyhow::Result<GitRelease> {
        let release = GitRest::get_latest_release(git_account, repo_name).await?;

        Ok(release)
    }

    pub fn update(path: &str, message: &str, local_branch: Branch, remote_branch: Branch, token: &str) -> anyhow::Result<()> {
        GitLocal::add_all(path)?;
        GitLocal::commit(path, message)?;
        GitLocal::push(path, local_branch, remote_branch, token)?;

        Ok(())
    }

    pub fn first_commit(path: &str, token: &str) -> anyhow::Result<()> {
        Self::update(path, "initial commit", Branch::Main, Branch::Main, token)?;

        Ok(())
    }

    pub fn add_submodule(repo_path: &str, remote_url: &str, local_path: &str) -> anyhow::Result<()> {
        GitLocal::submodule_add(repo_path, remote_url, local_path)?;
        GitLocal::submodule_update(repo_path)?;

        Ok(())
    }

    pub fn remote_add_origin(repo_path: &str, url: &str) -> anyhow::Result<()> {
        GitLocal::remote_add_origin(repo_path, url)?;

        Ok(())
    }

    pub fn add_gitignore(project_dir: &str, project_type: ProjectType) -> anyhow::Result<()> {
        let mut git_ignore_content = project_type.to_string();
    
        // Append '.env' to the gitignore content
        git_ignore_content.push_str("\n.env\n");
    
        let mut file = File::create(format!("{}/.gitignore", project_dir))?;
        file.write_all(git_ignore_content.as_bytes())?;
        
        Ok(())
    }

    pub fn find_git_root(starting_path: &Path) -> Option<PathBuf> {
        let mut current_path = starting_path.to_path_buf();
        let mut first_iteration = true;
    
        loop {
            if !first_iteration {
                let git_path = current_path.join(".git");
                if git_path.exists() {
                    return Some(current_path);
                }
            } else {
                first_iteration = false;
            }
    
            if !current_path.pop() {
                // We've reached the root and didn't find a .git folder
                return None;
            }
        }
    }
    

}

