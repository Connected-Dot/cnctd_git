use git2::{RemoteCallbacks, Cred, Repository, StatusOptions};
use serde_json::Value;



pub struct Git {
    token: String,
}

impl Git {
    pub fn new(token: String) -> Self {
        Self { token }
    } 

    pub async fn list_all_repos(&self) -> Result<(), reqwest::Error> {
        let url = "https://api.github.com/user/repos";
        let client = reqwest::Client::new();
        let response = client.get(url)
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "my-app")
            .send()
            .await?;

        let repos: Value = response.json().await?;
        println!("Repos: {:?}", repos);

        Ok(())
    }

    pub fn test_git2_auth(&self, repo_url: &str) -> Result<(), git2::Error> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::userpass_plaintext(username_from_url.unwrap(), &self.token)
        });

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Open a repository in read-only mode
        let repo = Repository::open_bare(repo_url)?;

        // Fetch from the remote
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["refs/heads/*:refs/heads/*"], Some(&mut fo), None)?;

        Ok(())
    }

    pub fn status(repo_path: &str) {
        // Open the repository
        let repo = match Repository::open(repo_path) {
            Ok(repo) => repo,
            Err(e) => {
                eprintln!("Could not open repository: {}", e);
                return;
            }
        };
    
        // Prepare status options (this step is optional)
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
    
        // Get the statuses
        let statuses = match repo.statuses(Some(&mut opts)) {
            Ok(statuses) => statuses,
            Err(e) => {
                eprintln!("Could not get statuses");
                return;
            }
        };
    
        // Loop through each status entry
        for entry in statuses.iter().filter(|e| e.status() != git2::Status::CURRENT) {
            let status = entry.status();
    
            if status.is_wt_new() {
                println!("New file: {}", entry.path().unwrap_or("unknown"));
            }
            if status.is_wt_modified() {
                println!("Modified file: {}", entry.path().unwrap_or("unknown"));
            }
            if status.is_wt_deleted() {
                println!("Deleted file: {}", entry.path().unwrap_or("unknown"));
            }
            // Add more conditions here based on what you want to check
        }
    }
}

