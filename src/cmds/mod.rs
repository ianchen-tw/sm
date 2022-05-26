mod config;

pub fn do_cmd_config() {
    use config::ConfigSubCmd;
    let sub_cmd = ConfigSubCmd::prompt();
    sub_cmd.run()
}
