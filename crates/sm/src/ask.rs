use filepath_complete::FilePathCompleter;
use inquire::{
    ui::{Color, RenderConfig, StyleSheet},
    validator::{StringValidator, Validation},
    Select, Text,
};

use crate::config::{AuthMethod, ConnectConfig};

pub fn init() {
    inquire::set_global_render_config(RenderConfig {
        default_value: StyleSheet::new().with_fg(Color::DarkGrey),
        ..Default::default()
    });
}

pub fn inquire_config(default: &ConnectConfig) -> ConnectConfig {
    fn ask(prompt: &str, default: &str) -> String {
        let answer = Text::new(prompt).with_default(default).prompt().unwrap();
        answer
    }

    let name = ask("Connection name", &default.name);
    let desc = ask("Description", &default.desc);
    let user = ask("User", &default.user);

    let server_addr = ask("Server address", &default.server_addr);

    let port = Text::new("Port")
        .with_default(&default.port.to_string())
        .with_validator(PortValidator)
        .with_help_message("port of the target machine")
        .prompt()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let auth_opts = vec!["none", "pem", "password"];
    let start_cursor: usize = match default.auth_method {
        AuthMethod::None => 0,
        AuthMethod::Pem(_) => 1,
        AuthMethod::Passwd => 2,
    };
    let message = format!("Authentication method [{}]", auth_opts[start_cursor]);
    let auth = match Select::new(&message, auth_opts)
        .with_starting_cursor(start_cursor)
        .prompt()
        .unwrap()
    {
        "none" => AuthMethod::None,
        "pem" => {
            let default_path = match &default.auth_method {
                AuthMethod::Pem(s) => s.clone(),
                _ => "~/.ssh/id_rsa".to_string(),
            };
            // let result = ask("Pem path", &default_path);
            let result = Text::new("Pem path")
                .with_default(&default_path)
                .with_autocomplete(FilePathCompleter::default())
                .prompt()
                .unwrap();
            AuthMethod::Pem(result)
        }
        "password" => AuthMethod::Passwd,
        _ => unreachable!(),
    };

    ConnectConfig {
        name,
        desc,
        user,
        server_addr,
        port,
        auth_method: auth,
    }
}

#[derive(Clone)]
struct PortValidator;
impl StringValidator for PortValidator {
    fn validate(
        &self,
        input: &str,
    ) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
        match input.parse::<u32>() {
            Ok(0..=65535) => Ok(Validation::Valid),
            Ok(_) => Ok(Validation::Invalid(
                "consider using port number between 0 to 65535".into(),
            )),
            Err(_) => Ok(Validation::Invalid(
                "invalid input, please provide a number between 0 and 65535.".into(),
            )),
        }
    }
}
