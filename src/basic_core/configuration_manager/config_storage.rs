use serde::{Deserialize, Serialize};

pub struct ConfigStorage {
    connection_info: ConnectionInfo
}

#[derive(Clone, Serialize, Deserialize)]  // save connection configuration
pub struct ConnectionInfo {
    pub ip: String,
    pub port: u16,
    pub username: String,
}
