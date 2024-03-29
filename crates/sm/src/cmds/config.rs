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
                let default = ConnectConfig::new(
                    "my custom connection",
                    "my custom description",
                    "root",
                    "192.168.1.1",
                    22,
                    AuthMethod::default(),
                );
                let result = ask::inquire_config(&default);
                result.show();
                sm_config.connections.push(result);
                sm_config.save_config();
            }
            ConfigSubCmd::Edit(mut sm_config) => {
                if sm_config.connections.is_empty() {
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
                if sm_config.connections.is_empty() {
                    info!("No config exists");
                    std::process::exit(0)
                }
                let index = sm_config.select();
                let target = &sm_config.connections[index];
                target.show();
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
