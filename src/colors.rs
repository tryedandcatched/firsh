use std::collections::HashMap;

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
