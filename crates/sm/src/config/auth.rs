use serde::{Deserialize, Serialize};

/// Scheme for ssh authentication
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum AuthMethod {
    None,
    Pem(String),
    Passwd,
}

impl Default for AuthMethod {
    fn default() -> Self {
        AuthMethod::Pem("~/.ssh/id_rsa".to_string())
    }
}

impl AuthMethod {
    // name of the auth method
    pub fn name(&self) -> &'static str {
        match self {
            AuthMethod::None => "none",
            AuthMethod::Passwd => "passwd",
            AuthMethod::Pem(_) => "pem",
        }
    }
}
