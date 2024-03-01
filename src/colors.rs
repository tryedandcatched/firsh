use std::collections::HashMap;
use crate::r#struct::{self, CommandLine};

pub fn translate_to_color(mut cli: String) -> String {
    let mut colors_map = HashMap::new();
    colors_map.insert("(black)", "\x1b[30m");
    colors_map.insert("(red)", "\x1b[31m");
    colors_map.insert("(green)", "\x1b[32m");
    colors_map.insert("(yellow)", "\x1b[33m");
    colors_map.insert("(blue)", "\x1b[34m");
    colors_map.insert("(magenta)", "\x1b[35m");
    colors_map.insert("(cyan)", "\x1b[36m");
    colors_map.insert("(white)", "\x1b[37m");
    colors_map.insert("(default)", "\x1b[39m");
    colors_map.insert("(bold)", "\x1b[1m");
    colors_map.insert("(underline)", "\x1b[4m");
    colors_map.insert("(blink)", "\x1b[5m");
    colors_map.insert("(reverse)", "\x1b[7m");
    colors_map.insert("(hidden)", "\x1b[8m");
    colors_map.insert("(black_bg)", "\x1b[40m");
    colors_map.insert("(red_bg)", "\x1b[41m");
    colors_map.insert("(green_bg)", "\x1b[42m");
    colors_map.insert("(yellow_bg)", "\x1b[43m");
    colors_map.insert("(blue_bg)", "\x1b[44m");
    colors_map.insert("(magenta_bg)", "\x1b[45m");
    colors_map.insert("(cyan_bg)", "\x1b[46m");
    colors_map.insert("(white_bg)", "\x1b[47m");
    colors_map.insert("(default_bg)", "\x1b[49m");
    colors_map.insert("(reset)", "\x1b[0m");

    cli.push_str("(reset)");

    for key in colors_map.keys() {
        cli = cli.replace(key, colors_map.get(key).unwrap());
    }

    cli
}

pub fn prompt_var(mut cli: String, env: &HashMap<String, String>) -> String {
    for (key, value) in env {
        cli = cli.replace(&format!("({})", key.to_lowercase()), &value);
    }

    cli
}

pub fn parse_arg(mut cli: String) -> Vec<String> {
    let mut args = Vec::new();
    let arg: Vec<&str> = cli.trim().split(" ").collect();
    if arg.len() < 1 {
        return args;
    }

    for mut i in 0..arg.len() {
        if arg[i].contains("\"") {
            let mut complete_arg = String::new();
            for j in i..arg.len() {
                if arg[j].contains("\"") {
                    args.push(arg[i..j].join(" "));
                    i = j;
                    break;
                } else {
                    complete_arg.push_str(arg[j]);
                    complete_arg.push(' ');
                }
                
            }
        } 
    }

    for a in arg {
        args.push(a.to_string());
    }

    args
}

pub fn parse_line(mut cli: String, env: &HashMap<String, String>) -> Vec<CommandLine> {
    let mut lines: Vec<CommandLine> = Vec::new();
    let cli = cli.split("\n").collect::<Vec<&str>>();
    for i in 0..cli.len() {
        if cli[i].contains("\\\\n") {
        }
        
    }
    for line in cli {
        let mut command = CommandLine::new();
        command.process = line.split(" ").collect::<Vec<&str>>()[0].to_string();
        command.args = parse_arg(line.to_string());
        lines.push(command);
    }
    println!("{:?}", lines);
    
    lines
}
