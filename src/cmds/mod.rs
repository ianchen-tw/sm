mod config;
use crate::config::SMConfig;
pub fn do_cmd_config(cur_config: SMConfig) {
    use config::ConfigSubCmd;
    let sub_cmd = ConfigSubCmd::prompt(cur_config);
    sub_cmd.run()
}
