use serde::{Deserialize, Serialize};

pub mod account;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum GitProvider {
    GitHub,
    GitLab,
    Bitbucket,
}
