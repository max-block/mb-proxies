use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProxyStatus {
    Unknown,
    Ok,
    Down,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ProxyProtocol {
    Socks5,
    Http,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub source: String,
    pub protocol: ProxyProtocol,
    pub status: ProxyStatus,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub created_at: DateTime<Utc>,
    pub checked_at: Option<DateTime<Utc>>,
    pub last_ok_at: Option<DateTime<Utc>>,
}

impl Proxy {
    pub fn new(source: &Source, host: String) -> Self {
        Self {
            id: None,
            source: source.id.clone(),
            protocol: source.protocol,
            status: ProxyStatus::Unknown,
            username: source.username.clone(),
            password: source.password.clone(),
            host,
            port: source.port,
            created_at: Utc::now(),
            checked_at: None,
            last_ok_at: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "_id")]
    pub id: String, // represents name
    pub url: String,
    pub protocol: ProxyProtocol,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub created_at: DateTime<Utc>,
    pub checked_at: Option<DateTime<Utc>>,
}

impl Source {
    pub fn new(id: String, url: String, protocol: ProxyProtocol, username: String, password: String, port: u16) -> Self {
        Self {
            id,
            url,
            protocol,
            username,
            password,
            port,
            created_at: Utc::now(),
            checked_at: None,
        }
    }
}
