use serde::{Deserialize, Serialize};

pub mod account;
pub mod api;
pub mod repo;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum GitProvider {
    GitHub,
    GitLab,
    Bitbucket,
}
