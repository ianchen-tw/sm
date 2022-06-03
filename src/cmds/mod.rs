mod config;
mod connect;

use crate::config::SMConfig;

pub fn do_cmd_config(cur_config: SMConfig) {
    use config::ConfigSubCmd;
    let sub_cmd = ConfigSubCmd::prompt(cur_config);
    sub_cmd.run()
}

pub fn do_connect_subcmd(sm_config: SMConfig) {
    println!("Connect to host:");
    let index = sm_config.select();
    let target = &sm_config.connections[index];
    connect::connect_host(&target);
}
