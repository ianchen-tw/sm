use crate::ask;
use crate::config::{AuthMethod, ConnectConfig, SMConfig};
use inquire::Select;
use log::info;

#[derive(Debug)]
pub enum ConfigSubCmd {
    Create(SMConfig),
    Edit(SMConfig),
    Delete(SMConfig),
    Show(SMConfig),
}

impl ConfigSubCmd {
    /// Get an subcommand by prompting to the user
    pub fn prompt(cur_config: SMConfig) -> ConfigSubCmd {
        let opts = vec!["Create", "Edit", "Show", "Delete"];
        match Select::new("Select cmd", opts).prompt().unwrap() {
            "Create" => ConfigSubCmd::Create(cur_config),
            "Edit" => ConfigSubCmd::Edit(cur_config),
            "Show" => ConfigSubCmd::Show(cur_config),
            "Delete" => ConfigSubCmd::Delete(cur_config),
            _ => unreachable!(),
        }
    }

    /// Start the subcommand
    pub fn run(self) {
        match self {
            ConfigSubCmd::Create(mut sm_config) => {
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
                sm_config.save_config();
            }
            ConfigSubCmd::Edit(mut sm_config) => {
                if sm_config.connections.len() == 0 {
                    info!("No config exists, create one before editing");
                    std::process::exit(0)
                }
                println!("do config edit");
                let target_index = sm_config.select();
                let result = ask::inquire_config(&sm_config.connections[target_index]);
                sm_config.connections[target_index] = result;
                sm_config.save_config();
            }
            ConfigSubCmd::Show(sm_config) => {
                if sm_config.connections.len() == 0 {
                    info!("No config exists");
                    std::process::exit(0)
                }
                let index = sm_config.select();
                let target = &sm_config.connections[index];
                println!("target: {:?}", target);
            }
            ConfigSubCmd::Delete(mut sm_config) => {
                println!("do config delete");
                let target_index = sm_config.select();
                sm_config.connections.remove(target_index);
                sm_config.save_config();
            }
        }
    }
}
