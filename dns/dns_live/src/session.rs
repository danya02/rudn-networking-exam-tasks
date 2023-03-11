use std::{net::Ipv4Addr, path::PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
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

#[derive(Serialize, Deserialize)]
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
pub enum Event {
    Request {
        request: Request,
        response: Response,
    },
    SwitchOutputMode(OutputMode),
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub server_ip: Ipv4Addr,
    pub name: String,
    pub record_type: RecordType,
}
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub server_ip: Ipv4Addr,
    pub name: String,
    pub record_type: RecordType,
}
