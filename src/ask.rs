use inquire::{
    ui::{Color, RenderConfig, StyleSheet},
    Text,
};

fn string_ask(prompt: &str, default: &str) -> String {
    let answer = Text::new(prompt).with_default(default).prompt().unwrap();
    return answer;
}

pub fn init() {
    inquire::set_global_render_config(get_render_config());
}

pub fn do_ask() {
    let name = string_ask("Connection name", "my custom connection");
    let desc = string_ask("Description", "my custom description");
    let user = string_ask("User", "root");

    println!("name -> {}", name);
    println!("desc -> {}", desc);
    println!("user -> {}", user);
}

fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();

    render_config.default_value = StyleSheet::new().with_fg(Color::DarkGrey);

    render_config
}
