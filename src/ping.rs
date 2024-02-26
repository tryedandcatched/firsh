// import modules to make an ping and do the function to ping an ip

use std::process::Command;

pub fn ping(ip: &str) {
    Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip)
        .output()
        .expect("failed to execute ping");

    println!("pinging {}", ip);
}
