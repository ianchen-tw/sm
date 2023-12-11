use super::auth::AuthMethod;
use comfy_table::Table;
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
        ConnectConfig::new(
            "ian",
            "desc",
            "yac",
            "192.168.1.1",
            22,
            AuthMethod::default(),
        )
    }
}

impl ConnectConfig {
    pub fn new(
        name: impl Into<String>,
        desc: impl Into<String>,
        user: impl Into<String>,
        server_addr: impl Into<String>,
        port: u32,
        auth_method: AuthMethod,
    ) -> Self {
        ConnectConfig {
            name: name.into(),
            desc: desc.into(),
            user: user.into(),
            server_addr: server_addr.into(),
            port,
            auth_method,
        }
    }
    pub fn show(&self) {
        let mut t = Table::new();
        t.set_header(vec!["Field", "Value"]);
        add_row(&mut t, "Name", &self.name);
        add_row(&mut t, "Description", &self.desc);
        add_row(&mut t, "Server Address", &self.server_addr);
        add_row(&mut t, "Connection Port", &self.port.to_string());
        add_row(&mut t, "Login User", &self.user);

        if let AuthMethod::Pem(path) = &self.auth_method {
            add_row(&mut t, "Authentication (pem)", path);
        } else {
            add_row(&mut t, "Authentication", self.auth_method.name());
        }

        println!("{t}");
    }
}

fn add_row(t: &mut Table, field: &str, val: &str) {
    t.add_row(vec![field, val]);
}
