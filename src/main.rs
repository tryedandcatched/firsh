use libc::{free, sigaddset, sigemptyset, sigprocmask, SIGINT, SIG_BLOCK, SIG_UNBLOCK};
use r#struct::prefix;
use std::collections::HashSet;
use std::process;
use termion::color;

mod colors;
mod detect_env;
mod helper;
mod llama;
mod ping;
mod r#struct;
mod versions;
use std::os;

use std::fmt::write;
use std::{collections::HashMap, env, fs, process::Command};

use rustyline::completion::Completer;
use rustyline::config::Configurer;
use rustyline::history::History;
use serde::{Deserialize, Serialize};

use rustyline::error::ReadlineError;
use rustyline::{history, Editor, Helper};

use crate::r#struct::{Config, WantFileType};
fn is_folder(path: &str) -> bool {
    if fs::metadata(path).is_err() {
        return false;
    }
    fs::metadata(path).unwrap().is_dir()
}

fn helper_index(pwd: &String, prefix: &Vec<prefix>) -> HashSet<helper::CommandHint> {
    let mut complete_list = HashSet::new();

    if fs::metadata(pwd).is_err() {
        return complete_list;
    }

    for file in fs::read_dir(pwd).unwrap() {
        let file_name = file.unwrap();
        let file_name = file_name.path();
        let file_name = file_name.file_name().unwrap();
        for p in prefix {
            if p.WantFileType == WantFileType::File {
                if !is_folder(format!("{}/{}", pwd, file_name.to_str().unwrap()).as_str()) {
                    complete_list.insert(helper::CommandHint::new(
                        format!("{} {}", p.process, file_name.to_str().unwrap()).as_str(),
                        format!("{} {}", p.process, file_name.to_str().unwrap()).as_str(),
                    ));
                }
            }
            if p.WantFileType == WantFileType::Dir {
                if is_folder(format!("{}/{}", pwd, file_name.to_str().unwrap()).as_str()) {
                    complete_list.insert(helper::CommandHint::new(
                        format!("{} {}", p.process, file_name.to_str().unwrap()).as_str(),
                        format!("{} {}", p.process, file_name.to_str().unwrap()).as_str(),
                    ));
                }
            }
        }
    }

    complete_list
}

fn main() {
    //get args
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        match args[1].as_str() {
            "test" => {
                let tests_path = vec![
                    r#"cd "data""#,
                    r#"ping 8.8.8.8"#,
                    r#"ffmpeg -i "test.mp4" -o "test.mp3" " je "#,
                    r#"cd /home/wyene"#,
                    r#"cd /home/wyene/mhome/project/firsh"#,
                    r#"cd /home/wyene/mhome/project/firsh/
git status
git fetch"#,
                ];
                for test in tests_path {
                    println!("{:#?}", colors::parse_line(test.into(), &HashMap::new()));
                }

                return;
            }
            _ => {}
        }
    }

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
    //let all_file = Command::new("find")
    //.arg(".")
    //.output()
    //.expect("failed to execute process");
    //let output = String::from_utf8(all_file.stdout.clone()).unwrap();
    //for line in output.lines() {
    //let line = line.replace("./", "cd ");
    //complete_list.insert(helper::CommandHint::new(
    //&line,
    //&line,
    //));
    //}

    let path_dirs = env_var.get("PATH").unwrap();
    for path in path_dirs.split(":") {
        if is_folder(&path) {
            for file_name in fs::read_dir(&path).unwrap() {
                let file_name = file_name.unwrap();
                let path = format!(
                    "{}",
                    file_name.path().file_name().unwrap().to_str().unwrap()
                );
                complete_list.insert(helper::CommandHint::new(&path, &path));
            }
        }
    }

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
            let mut complete_list = HashSet::new();

            if is_folder(format!("{}/{}", actual_path, &file_name.to_str().unwrap()).as_str()) {
                complete_list.insert(helper::CommandHint::new(
                    format!("cd {}", file_name.to_str().unwrap()).as_str(),
                    format!("cd {}", file_name.to_str().unwrap()).as_str(),
                ));
            }
        }

        let coding_env = detect_env::detect_work_env();

        println!("");
        let prompt = config.style.prompt.clone();

        let prompt = colors::prompt_var(prompt, &env_var);
        let prompt = colors::translate_to_color(prompt);
        let prompt = prompt.replace(config.clone().shell.unwrap().pwd.as_str(), "~");
        if coding_env != "Unknown" {
            let ver = version.get(&coding_env).unwrap();
            println!("{}: {}", coding_env, ver);
        }
        let readline = rl.readline(&prompt);
        let prefix: Vec<r#struct::prefix> = vec![
            r#struct::prefix {
                process: "nvim".into(),
                WantFileType: WantFileType::File,
            },
            r#struct::prefix {
                process: "cd".into(),
                WantFileType: WantFileType::Dir,
            },
            r#struct::prefix {
                process: "ls".into(),
                WantFileType: WantFileType::Dir,
            },
            r#struct::prefix {
                process: "vim".into(),
                WantFileType: WantFileType::File,
            },
            r#struct::prefix {
                process: "helix".into(),
                WantFileType: WantFileType::File,
            },
        ];

        let pwd = env_var.get("PWD").unwrap().to_string();
        complete_list = helper_index(&pwd, &prefix);
        match readline {
            Ok(mut line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                line = line.trim().to_string();
                let pwd = env_var.get("PWD").unwrap().to_string();
                drop(complete_list);

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
                    continue;
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
                    continue;
                } else {
                    if line.len() < 1 {
                        continue;
                    }

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

                            command.env("SHELL", "/bin/bash");
                            command.stdin(std::process::Stdio::inherit());
                            command.stdout(std::process::Stdio::inherit());
                            command.stderr(std::process::Stdio::inherit());

                            for arg in args {
                                command.arg(arg);
                            }
                            //wait user input

                            let status = command.status();
                            match status {
                                Ok(status) => if status.success() {},
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
                    complete_list = helper_index(&pwd, &prefix);
                    let hlper = helper::DIYHinter {
                        hints: complete_list.clone(),
                    };
                    rl.set_helper(Some(hlper));
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
