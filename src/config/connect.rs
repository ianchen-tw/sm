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

impl ConnectConfig {
    pub fn show(&self) {
        let mut t = Table::new();
        t.set_header(vec!["Field", "Value"]);
        add_row(&mut t, "Name", &self.name);
        add_row(&mut t, "Description", &self.desc);
        add_row(&mut t, "Server Address", &self.server_addr);
        add_row(&mut t, "Connection Port", &self.port.to_string());
        add_row(&mut t, "Login User", &self.user);

        if let AuthMethod::Pem(path) = &self.auth_method {
            add_row(&mut t, "Authentication (pem)", &path);
        } else {
            add_row(
                &mut t,
                "Authentication",
                &self.auth_method.name().to_string(),
            );
        }

        println!("{t}");
    }
}

fn add_row(t: &mut Table, field: &str, val: &String) {
    t.add_row(vec![field, val.as_str()]);
}
