use crate::{api::rest::GitRest, account::GitAccount};

pub struct GitRepo {
    url: String,
}

// impl GitRepo {
//     pub async fn new(git_account: &GitAccount, repo_name: &str, description: Option<&str>, is_private: bool) -> anyhow::Result<Self> {
//         let GitRest::create_repo(git_account, repo_name, description, is_private).await?
//     }
// }