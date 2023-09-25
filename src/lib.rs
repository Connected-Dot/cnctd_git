use chrono::{NaiveDateTime, DateTime};
use serde::{Deserialize, Serialize, Deserializer};

pub mod account;
pub mod api;
pub mod repo;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum GitProvider {
    GitHub,
    GitLab,
    Bitbucket,
}

fn from_str_to_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let datetime = DateTime::parse_from_rfc3339(&s)
        .map_err(serde::de::Error::custom)?;
    Ok(datetime.naive_utc())
}
