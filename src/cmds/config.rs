use crate::ask;
use crate::config::{AuthMethod, ConnectConfig};

#[derive(Debug)]
pub enum ConfigSubCmd {
    Create,
    Edit,
    Delete,
}
impl ConfigSubCmd {
    /// Get an subcommand by prompting to the user
    pub fn prompt() -> ConfigSubCmd {
        use inquire::Select;
        let opts = vec!["Create", "Edit", "Delete"];
        match Select::new("Select cmd", opts).prompt().unwrap() {
            "Create" => ConfigSubCmd::Create,
            "Edit" => ConfigSubCmd::Edit,
            "Delete" => ConfigSubCmd::Delete,
            _ => unreachable!(),
        }
    }

    /// Start the subcommand
    pub fn run(self) {
        match self {
            ConfigSubCmd::Create => config_create(),
            ConfigSubCmd::Edit => {
                println!("do config edit");
                let target = select_config();
                let result = ask::inquire_config(&target);
                replace_config(&result);
            }
            ConfigSubCmd::Delete => {
                println!("do config delete");
                let target = select_config();
                remove_config(&target);
            }
        }
    }
}

fn config_create() {
    let default_config = ConnectConfig {
        name: "my custom connection".to_string(),
        desc: "my custom description".to_string(),
        user: "root".to_string(),
        server_addr: "192.168.1.1".to_string(),
        port: 22,
        auth_method: AuthMethod::default(),
    };
    let final_config = ask::inquire_config(&default_config);
    println!("final config: {:?}", final_config);

    println!("Write new connection");
    // TODO: Write to connection file
}

fn select_config() -> ConnectConfig {
    return ConnectConfig {
        name: "my custom connection".to_string(),
        desc: "my custom description".to_string(),
        user: "root".to_string(),
        server_addr: "192.168.1.1".to_string(),
        port: 22,
        auth_method: AuthMethod::default(),
    };
}

// Replace existing config with provided one
fn replace_config(_cfg: &ConnectConfig) {
    println!("Config replaced!")
}

// Replace existing config with provided one
fn remove_config(_cfg: &ConnectConfig) {
    println!("Config removed!")
}
