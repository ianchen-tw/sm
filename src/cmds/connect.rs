use crate::config::AuthMethod;
use crate::config::ConnectConfig;

use std::vec::Vec;

pub fn connect_host(config: &ConnectConfig) {
    println!("Connect to config: {}", &config.name);

    let mut cmd = Vec::new();

    cmd.push("ssh");

    if let AuthMethod::Pem(location) = &config.auth_method {
        cmd.push("-i");
        cmd.push(&location.as_str());
    }

    // Host
    let s = format!("{}@{}:{}", config.user, config.server_addr, config.port);
    cmd.push(s.as_str());

    println!("{:?}", cmd);
}
