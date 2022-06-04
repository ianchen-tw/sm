use anyhow::Context;

use crate::config::AuthMethod;
use crate::config::ConnectConfig;

use std::process;
use std::vec::Vec;

pub fn connect_host(config: &ConnectConfig) {
    let mut args = Vec::new();

    if let AuthMethod::Pem(location) = &config.auth_method {
        args.push("-i");
        args.push(&location.as_str())
    }

    let port = format!("{}", config.port);
    args.push("-p");
    args.push(port.as_str().clone());

    args.push("-l");
    args.push(&config.user);

    args.push(&config.server_addr.as_str());

    let mut sp = process::Command::new("ssh")
        .args(args)
        .spawn()
        .with_context(|| "Unable to connect to host")
        .unwrap();
    sp.wait().unwrap();
}
