use inquire::{
    ui::{Color, RenderConfig, StyleSheet},
    Text,
};

use crate::config::ConnectConfig;

fn port_validator(input: &str) -> Result<(), String> {
    return match input.parse::<u32>() {
        Ok(0..=65535) => Ok(()),
        Ok(_) => Err("consider using port number between 0 to 65535".to_string()),
        Err(_) => Err("invalid input, please provide a number between 0 and 65535.".to_string()),
    };
}

fn inquire_config(default: &ConnectConfig) -> ConnectConfig {
    fn ask(prompt: &str, default: &str) -> String {
        let answer = Text::new(prompt).with_default(default).prompt().unwrap();
        return answer;
    }

    let name = ask("Connection name", &default.name);
    let desc = ask("Description", &default.desc);
    let user = ask("User", &default.user);

    let server_addr = ask("Server address", &default.server_addr);

    let port = Text::new("Port")
        .with_default(&default.port.to_string())
        .with_validator(&port_validator)
        .with_help_message("port of the target machine")
        .prompt()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    // TODO: validate input
    let auth = ask("Authentication method", &default.auth);

    // TODO: skip if using password
    let pem_path = ask("Pem path", &default.pem_path);

    return ConnectConfig {
        name: name,
        desc: desc,
        user: user,
        server_addr: server_addr,
        port: port,
        pem_path: pem_path,
        auth: auth,
    };
}

pub fn init() {
    inquire::set_global_render_config(get_render_config());
}

pub fn do_ask() {
    let default_config = ConnectConfig {
        name: "my custom connection".to_string(),
        desc: "my custom description".to_string(),
        user: "root".to_string(),
        server_addr: "192.168.1.1".to_string(),
        port: 22,
        pem_path: "~/.ssh/good".to_string(),
        auth: "pem".to_string(),
    };

    let final_config = inquire_config(&default_config);
    println!("final config: {:?}", final_config);
}

fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();
    render_config.default_value = StyleSheet::new().with_fg(Color::DarkGrey);

    render_config
}
