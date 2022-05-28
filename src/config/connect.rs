use super::auth::AuthMethod;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectConfig {
    pub name: String,
    pub desc: String,
    pub user: String,
    pub server_addr: String,
    pub port: u32,
    pub auth_method: AuthMethod,
}

impl Default for ConnectConfig {
    fn default() -> Self {
        ConnectConfig {
            name: "ian".to_string(),
            desc: "desc".to_string(),
            user: "yac".to_string(),
            server_addr: "192.168.1.1".to_string(),
            port: 22,
            auth_method: AuthMethod::default(),
        }
    }
}
