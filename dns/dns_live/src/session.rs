use std::{
    net::IpAddr,
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
};
use trust_dns_client::rr::RecordType;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_requests: Vec<RequestLogEntry>,
    pub current_output_mode: OutputMode,
}

pub async fn get_session(key: &str) -> Option<Session> {
    let mut path = PathBuf::new();
    path.push("sessions");
    path.push(key);
    path.set_extension("json");
    let mut file = tokio::fs::File::open(path).await.ok()?;
    let mut data = String::new();
    file.read_to_string(&mut data).await.ok()?;
    serde_json::from_str(&data).ok()?
}

pub async fn set_session(key: &str, new_session: Session) -> Option<()> {
    let mut path = PathBuf::new();
    path.push("sessions");
    path.push(key);
    path.set_extension("json");
    let mut options = OpenOptions::new();
    let mut file = options.write(true).open(path).await.ok()?;
    let data = serde_json::to_vec_pretty(&new_session).ok()?;
    file.write_all(&data).await.ok()?;
    Some(())
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum OutputMode {
    /// As Display
    Classic,
    /// As Debug
    Rust,
}

#[derive(Serialize, Deserialize)]
pub struct RequestLogEntry {
    pub when: DateTime<Utc>,
    pub what: Event,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    Request {
        request: Request,
        response: ResponseResult,
    },
    SwitchOutputMode {
        new_mode: OutputMode,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub server_ip: String,
    pub name: String,
    pub record_type: RecordType,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseResult {
    Ok {
        resp: Response,
    },
    QueryError {
        err: String,
    },

    /// Tried querying a recursive server
    ForbiddenRecursion {
        addr: IpAddr,
    },
    /// Failed to parse the request IP address
    InvalidRequestIpAddr {
        addr: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub text: String,
    pub mode: OutputMode,
}
