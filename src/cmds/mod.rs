mod config;

pub fn do_cmd_config() {
    use config::ConfigSubCmd;
    match ConfigSubCmd::prompt() {
        ConfigSubCmd::Create => config::do_config_create(),
        ConfigSubCmd::Edit => {}
        ConfigSubCmd::Delete => {}
    }
}
