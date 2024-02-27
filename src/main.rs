use libc::{sigaddset, sigemptyset, sigprocmask, SIGINT, SIG_BLOCK, SIG_UNBLOCK};
use std::collections::HashSet;
use termion::color;

mod colors;
mod detect_env;
mod helper;
mod llama;
mod ping;
mod r#struct;
mod versions;

use std::fmt::write;
use std::{collections::HashMap, env, fs, process::Command};

use rustyline::completion::Completer;
use rustyline::config::Configurer;
use rustyline::history::History;
use serde::{Deserialize, Serialize};

use rustyline::error::ReadlineError;
use rustyline::{history, Editor, Helper};

use crate::r#struct::Config;
fn is_folder(path: &str) -> bool {
    if fs::metadata(path).is_err() {
        return false;
    }
    fs::metadata(path).unwrap().is_dir()
}

fn main() {
    unsafe {
        // Create an empty signal mask
        let mut mask: libc::sigset_t = std::mem::zeroed();
        sigemptyset(&mut mask);
        // Add the SIGINT signal to the signal mask
        sigaddset(&mut mask, SIGINT);
        // Block the SIGINT signal using the signal mask
        sigprocmask(
            SIG_BLOCK,
            &mask as *const libc::sigset_t,
            std::ptr::null_mut(),
        );
    }
    let line_config = rustyline::config::Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::config::CompletionType::List)
        .edit_mode(rustyline::config::EditMode::Vi)
        .build();

    let hlper: helper::DIYHinter = helper::DIYHinter {
        hints: helper::diy_hints(),
    };

    let mut rl =
        rustyline::Editor::<helper::DIYHinter, history::DefaultHistory>::with_config(line_config)
            .unwrap();
    rl.set_bell_style(rustyline::config::BellStyle::None);
    rl.set_auto_add_history(true);
    rl.set_completion_type(rustyline::config::CompletionType::List);
    rl.set_helper(Some(hlper.clone()));

    rl.bind_sequence(
        rustyline::KeyEvent::alt('p'),
        rustyline::Cmd::HistorySearchBackward,
    );

    rl.bind_sequence(
        rustyline::KeyEvent::alt('n'),
        rustyline::Cmd::HistorySearchForward,
    );

    rl.bind_sequence(rustyline::KeyEvent::ctrl('e'), rustyline::Cmd::CompleteHint);

    rl.bind_sequence(rustyline::KeyEvent::ctrl('c'), rustyline::Cmd::Abort);

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let config_path = format!("{}/shell.toml", env::var("HOME").unwrap());
    if !fs::metadata(&config_path).is_ok() {
        fs::write(
            &config_path,
            r#"[style]
prompt = "(green)[(red)(pwd)(green)](blue)\n)"

[shell]
pwd = "/home/wyene/mhome"
aliases = {ls = "ls --color=always", ping = "ping -c 1", ll = "ls -l --color=always", la = "ls -a --color=always"}

#[llama]
#llama_path = "your path"
#model_path = "your path"
#prompt = "you'r an AI assistant that work inside an terminal the user will ask for help (probably command), you will only answer by 1 line of bash code. you need to make it the more concise way possible do not explain do not format just write the asked command no more no less; user problem: "
"#,
        )
        .unwrap();
    }
    let config: Config =
        toml::from_str(fs::read_to_string(&config_path).unwrap().as_str()).unwrap();

    let mut env_var = HashMap::new();
    for (key, value) in env::vars() {
        env_var.insert(key, value);
    }

    let mut complete_list = HashSet::new();

    if let Some(shell) = &config.shell {
        env_var.insert("PWD".to_string(), shell.pwd.to_string());
    }

    let version = versions::get_all_versions();
    loop {
        env::set_current_dir(env_var.get("PWD").unwrap()).unwrap();
        for path in fs::read_dir(".").unwrap() {
            let file_name = path.unwrap();
            let file_name = file_name.path();
            let file_name = file_name.file_name().unwrap();
            let actual_path = env_var.get("PWD").unwrap();

            if is_folder(format!("{}/{}", actual_path, &file_name.to_str().unwrap()).as_str()) {
                complete_list.insert(helper::CommandHint::new(
                    format!("cd {}", file_name.to_str().unwrap()).as_str(),
                    format!("cd {}", file_name.to_str().unwrap()).as_str(),
                ));
            }
        }

        let hlper = helper::DIYHinter {
            hints: complete_list.clone(),
        };
        rl.set_helper(Some(hlper));

        let coding_env = detect_env::detect_work_env();

        let prompt = config.style.prompt.clone();
        let prompt = colors::prompt_var(prompt, &env_var);
        let prompt = colors::translate_to_color(prompt);
        let prompt = prompt.replace(config.clone().shell.unwrap().pwd.as_str(), "~");
        if coding_env != "Unknown" {
            let ver = version.get(&coding_env).unwrap();
            println!("{}: {}", coding_env, ver);
        }
        let readline = rl.readline(&prompt);
        match readline {
            Ok(mut line) => {
                rl.add_history_entry(line.as_str()).unwrap();

                if line == "exit" || line == "quit" {
                    break;
                }

                if line.starts_with("cd") {
                    let path = line.replace("cd ", "");

                    if path == ".." {
                        let actual_path = env_var.get("PWD").unwrap();
                        let split_path = actual_path.split("/").collect::<Vec<&str>>();
                        let new_path = split_path[0..split_path.len() - 1].join("/");
                        if is_folder(&new_path) {
                            env_var.insert("PWD".to_string(), new_path);
                        }
                    } else {
                        let pwd = env_var.get("PWD").unwrap();
                        if is_folder(&path) {
                            env_var.insert("PWD".to_string(), format!("{}/{}", pwd, path));
                        } else {
                            if is_folder(&path) {
                                env_var.insert("PWD".to_string(), path);
                            }
                        }
                    }
                }
                if line.starts_with("#") {
                    if config.clone().llama.is_none() {
                        println!(
                            "{}",
                            colors::translate_to_color(
                                "Please set the llama model path in shell.toml".to_string()
                            )
                        );
                        continue;
                    }
                    let prompt = format!(
                        "{}\"{}\". You'r answer as an AI:",
                        config.clone().llama.unwrap().prompt,
                        line.replace("#", "")
                    );
                    let llama_path = config.clone().llama.unwrap().llama_path;
                    let model_path = config.clone().llama.unwrap().model_path;
                    let llama_output = llama::execute(&llama_path, &model_path, &prompt);
                    println!("{}", colors::translate_to_color(llama_output));
                } else {
                    if config.clone().shell.unwrap().aliases.is_some() {
                        let aliases = config.clone().shell.unwrap().aliases.unwrap();
                        if aliases.contains_key(line.as_str()) {
                            line = aliases.get(line.as_str()).unwrap().to_string();
                        }
                    }
                    match line.as_str() {
                        _ => {
                            env::set_current_dir(env_var.get("PWD").unwrap()).unwrap();

                            let command = line.split(" ").collect::<Vec<&str>>();
                            let args = command[1..].to_vec();
                            let mut command = Command::new(command[0]);
                            for (key, value) in env_var.clone() {
                                command.env(key, value);
                            }

                            for arg in args {
                                command.arg(arg);
                            }

                            let status = command.status();
                            match status {
                                Ok(status) => {
                                    if status.success() {
                                        let _output = command.output().unwrap();
                                    }
                                }
                                Err(error) => {
                                    if error.kind() == std::io::ErrorKind::NotFound {
                                        println!("{} not found", line);
                                    }
                                    if error.kind() == std::io::ErrorKind::PermissionDenied {
                                        println!("{} permission denied", line);
                                    }
                                    if error.kind() == std::io::ErrorKind::OutOfMemory {
                                        println!("{} out of memory", line);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => {}
            Err(_) => {}
        }
    }

    rl.save_history("history.txt").unwrap();
}

// todo!("when an user enter the name of an folder it cd to that folder");
