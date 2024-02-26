use std::collections::HashMap;
use std::process::Command;
use crate::r#struct::{self, programming_language};

fn cargo_version() -> String {
    let output = Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8(output.stdout).unwrap();
    let output = output.split(" ").collect::<Vec<&str>>();
    output[1].to_string()
}

fn node_version() -> String {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    String::from_utf8(output.stdout).unwrap()
}

fn npm_version() -> String {
    let output = Command::new("npm")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    String::from_utf8(output.stdout).unwrap()
}

fn go_version() -> String {
    let output = Command::new("go")
        .arg("version")
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8(output.stdout).unwrap();
    output.split(" ").collect::<Vec<&str>>()[2]
        .to_string()
        .replace("go", "")
}

fn python_version() -> String {
    let output = Command::new("python")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    let output = String::from_utf8(output.stdout).unwrap();
    output.split(" ").collect::<Vec<&str>>()[1].to_string()
}

pub fn get_all_versions() -> HashMap<String, String> {
    let mut versions = HashMap::new();
    versions.insert(programming_language::Rust.to_string(), cargo_version());
    versions.insert(programming_language::Go.to_string(), go_version());
    versions.insert(programming_language::Python.to_string(), python_version());
    versions.insert(programming_language::Node.to_string(), node_version());
    versions
}
