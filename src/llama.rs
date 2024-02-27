use std::io::{BufRead, BufReader};
use std::process::{self, Command, Stdio};
pub fn execute(llama_path: &String, model_path: &String, prompt: &String)  -> String {
    let mut command = process::Command::new("prime");
    let command = command
        .arg(llama_path)
        .arg("--model")
        .arg(model_path)
        .arg("--prompt")
        .arg(prompt);

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Create readers for the output streams
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    // Iterate over output lines and process them
    let mut start_printing = false;
    let mut output = String::new();
    for line_result in stdout_reader.lines() {
        let line = line_result.unwrap();
        let mut processed_line = line.replace(prompt, "").replace("\n", "").replace("`", "");
        processed_line = processed_line.trim().to_string();

        if line.contains(prompt) {
            start_printing = true;
        } 

        if start_printing {
            output = output + &processed_line + "\n";
        }

    }

    // Handle potential errors from the child process
    let exit_status = child.wait().unwrap();
    if !exit_status.success() {
        for line_result in stderr_reader.lines() {
            let error_line = line_result.unwrap();
            eprintln!("Error: {}", error_line);
        }
    }

    output
}
