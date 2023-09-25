use std::path::Path;

use git2::{Repository, IndexAddOption, Signature, Commit};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitLocal {
    name: String,
    email: String,
}

#[derive(Debug)]
pub enum Branch {
    Main,
    Development,
    Feature(String),
}

impl Branch {
    pub fn as_str(&self) -> String {
        match self {
            Branch::Main => "main".to_string(),
            Branch::Development => "development".to_string(),
            Branch::Feature(name) => format!("feature/{}", name),
        }
    }
}

impl GitLocal {
    fn get_signature(repo: &Repository) -> anyhow::Result<Signature> {
        let config = repo.config()?;
        let name = config.get_string("user.name")?;
        let email = config.get_string("user.email")?;

        Ok(Signature::now(&name, &email)?)
    }

    pub fn init(path: &str) -> anyhow::Result<Repository> {
        let repo = Repository::init(path)?;
        
        // Make an initial empty commit so that a branch is created
        {
            let sig = Self::get_signature(&repo)?;
            let tree_id = {
                let mut index = repo.index()?;
                index.write_tree()?
            };
            let tree = repo.find_tree(tree_id)?;
            repo.commit(Some("HEAD"), &sig, &sig, "Initial empty commit", &tree, &[])?;
        }
    
        // Now rename the default 'master' branch to 'main'
        if let Ok(mut master_branch) = repo.find_branch("master", git2::BranchType::Local) {
            master_branch.rename("main", false)?;
        }

        Ok(repo)
    }
    

    pub fn remote_add_origin(repo_path: &str, url: &str) -> anyhow::Result<()> {
        let repo = Repository::open(Path::new(repo_path))?;
        repo.remote("origin", url)?;

        Ok(())
    }
    

    pub fn add_all(path: &str) -> anyhow::Result<()> {
        let repo = Repository::open(Path::new(path))?;
        let mut index = repo.index()?;
        index.add_all(["*"], IndexAddOption::DEFAULT, None)?;
        index.write()?;
        
        Ok(())
    }

    pub fn commit(path: &str, message: &str) -> anyhow::Result<()> {
        let repo = Repository::open(Path::new(path))?;
        let mut index = repo.index()?;
        let oid = index.write_tree()?;
        let signature = Self::get_signature(&repo)?;
        let tree = repo.find_tree(oid)?;
    
        let parent_commit_result = repo.head().and_then(|head| head.peel_to_commit());
    
        let parents: Vec<Commit> = match parent_commit_result {
            Ok(parent_commit) => vec![parent_commit],
            Err(_) => vec![],  // No parent commit for the first commit
        };
    
        let parents_refs: Vec<&_> = parents.iter().collect();
    
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents_refs,
        )?;
    
        Ok(())
    }
    
    

    pub fn push(repo_path: &str, local_branch: Branch, remote_branch: Branch, token: &str) -> anyhow::Result<()> {
        let repo = Repository::open(Path::new(repo_path))?;
        let mut remote = repo.find_remote("origin")?;
        
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            git2::Cred::userpass_plaintext(token, "")
        });
        
        let mut push_options = git2::PushOptions::new();
        push_options.remote_callbacks(callbacks);
        
        let refspec = format!(
            "refs/heads/{}:refs/heads/{}",
            local_branch.as_str(),
            remote_branch.as_str()
        );
        
        remote.push(&[&refspec], Some(&mut push_options))?;
        
        Ok(())
    }
    

    pub fn submodule_add(repo_path: &str, remote_url: &str, path: &str) -> anyhow::Result<()> {
        let repo = Repository::open(Path::new(repo_path))?;
        let mut submodule = repo.submodule(remote_url, Path::new(path), true)?;
        let mut opts = git2::SubmoduleUpdateOptions::new();
        submodule.add_finalize()?;
        submodule.update(true, Some(&mut opts))?;

        Ok(())
    }

    pub fn submodule_update(repo_path: &str) -> anyhow::Result<()> {
        let repo = Repository::open(Path::new(repo_path))?;
        let mut opts = git2::SubmoduleUpdateOptions::new();
        for mut sub in repo.submodules()? {
            sub.update(true, Some(&mut opts))?;
        }
        Ok(())
    }

}