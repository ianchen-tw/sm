use inquire::{
    ui::{Color, RenderConfig, StyleSheet},
    Select, Text,
};
use std::fmt;
#[derive(Debug)]
enum ConfigSubCmd {
    Create,
    Edit,
    Delete,
}

impl ConfigSubCmd {
    fn prompt() -> ConfigSubCmd {
        let opts = vec!["Create", "Edit", "Delete"];
        match Select::new("Select cmd", opts).prompt().unwrap() {
            "Create" => ConfigSubCmd::Create,
            "Edit" => ConfigSubCmd::Edit,
            "Delete" => ConfigSubCmd::Delete,
            _ => unreachable!(),
        }
    }
}

pub fn do_cmd_config() {
    let sub_cmd = ConfigSubCmd::prompt();
    println!("Good {:?}", sub_cmd)
}
