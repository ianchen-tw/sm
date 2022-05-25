use crate::ask;
use crate::config::{AuthMethod, ConnectConfig};

#[derive(Debug)]
pub enum ConfigSubCmd {
    Create,
    Edit,
    Delete,
}
impl ConfigSubCmd {
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
}

pub fn do_config_create() {
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
