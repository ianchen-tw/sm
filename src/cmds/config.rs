use crate::ask;
use crate::config::{AuthMethod, ConnectConfig, SMConfig};
#[derive(Debug)]
pub enum ConfigSubCmd {
    Create(SMConfig),
    Edit(SMConfig),
    Delete(SMConfig),
}
impl ConfigSubCmd {
    /// Get an subcommand by prompting to the user
    pub fn prompt(cur_config: SMConfig) -> ConfigSubCmd {
        use inquire::Select;
        let opts = vec!["Create", "Edit", "Delete"];
        match Select::new("Select cmd", opts).prompt().unwrap() {
            "Create" => ConfigSubCmd::Create(cur_config),
            "Edit" => ConfigSubCmd::Edit(cur_config),
            "Delete" => ConfigSubCmd::Delete(cur_config),
            _ => unreachable!(),
        }
    }

    /// Start the subcommand
    pub fn run(self) {
        match self {
            ConfigSubCmd::Create(val) => config_create(val),
            ConfigSubCmd::Edit(_val) => {
                println!("do config edit");
                let target = select_config();
                let result = ask::inquire_config(&target);
                replace_config(&result);
            }
            ConfigSubCmd::Delete(_val) => {
                println!("do config delete");
                let target = select_config();
                remove_config(&target);
            }
        }
    }
}

fn config_create(mut sm_config: SMConfig) {
    let default = ConnectConfig {
        name: "my custom connection".to_string(),
        desc: "my custom description".to_string(),
        user: "root".to_string(),
        server_addr: "192.168.1.1".to_string(),
        port: 22,
        auth_method: AuthMethod::default(),
    };
    let result = ask::inquire_config(&default);
    println!("final config: {:?}", result);
    sm_config.connections.push(result);

    // TODO: Write to connection file
    println!("sm config: {:?}", sm_config);
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
