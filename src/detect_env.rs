use std::fs;
use std::fmt;
use std::collections::HashMap;

use crate::r#struct::programming_language;

fn detect_rust() -> programming_language {
    if fs::metadata("Cargo.toml").is_ok() {
        return programming_language::Rust;
    }
    programming_language::Unknown
}

fn detect_go() -> programming_language {
    if fs::metadata("go.mod").is_ok() {
        return programming_language::Go;
    }
    programming_language::Unknown
}

fn detect_node() -> programming_language {
    if fs::metadata("package.json").is_ok() {
        return programming_language::Node;
    }
    programming_language::Unknown
}

fn detect_python() -> programming_language {
    if fs::metadata("requirements.txt").is_ok() || fs::metadata("pyproject.toml").is_ok() || fs::metadata("main.py").is_ok() {
        return programming_language::Python;
    }
    programming_language::Unknown
}

pub fn detect_work_env() -> String {

    let mut envs = HashMap::new();
    envs.insert(programming_language::Rust.to_string(), detect_rust());
    envs.insert(programming_language::Go.to_string(), detect_go());
    envs.insert(programming_language::Python.to_string(), detect_python());
    envs.insert(programming_language::Node.to_string(), detect_node());

    for (env, language) in envs {
        if language != programming_language::Unknown {
            return env.to_string();
        }
    }
    "Unknown".to_string()

}

