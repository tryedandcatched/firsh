use std::collections::HashMap;
use std::fmt;


use serde::Deserialize;

pub struct command {
    name: String,
    args: Vec<String>,
    envs: Vec<(String, String)>,
    cwd: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub style: Style,
    pub shell: Option<Shell>,
}

#[derive(Deserialize, Clone)]
pub struct Style {
    pub prompt: String,
}

#[derive(Deserialize, Clone)]
pub struct Shell {
    pub pwd: String,
    pub aliases: Option<HashMap<String, String>>,
}


#[derive(PartialEq, Eq)]
pub enum programming_language {
    Rust,
    Go,
    Python,
    Node,
    Unknown,
}


impl fmt::Display for programming_language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            programming_language::Rust => write!(f, "Rust"),
            programming_language::Go => write!(f, "Go"),
            programming_language::Python => write!(f, "Python"),
            programming_language::Node => write!(f, "Node"),
            programming_language::Unknown => write!(f, "Unknown"),
        }
    }
}

impl programming_language {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "rust" => programming_language::Rust,
            "go" => programming_language::Go,
            "python" => programming_language::Python,
            "node" => programming_language::Node,
            _ => programming_language::Unknown,
        }
        
    }
}



