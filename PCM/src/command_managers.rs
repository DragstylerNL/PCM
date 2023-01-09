use serde_derive::Deserialize;
use std::process::Command;

const COMMAND_KEY: &str = "PCM";

#[derive(Deserialize)]
pub struct Commands {
    command: String,
    path: String,
}

pub fn check_commandword(data: &[Commands], input: &String) -> String {
    let mut r_string = input.to_string();
    if input.contains(COMMAND_KEY) {
        r_string = read_lauchables(data, input);
    }
    return r_string;
}

fn read_lauchables(data: &[Commands], input: &String) -> String {
    let mut r_string = input.to_string();
    for i in 0..data.len() {
        if input.contains(&data[i].command) {
            let _ = Command::new(&data[i].path).spawn();
            r_string = String::new();
        }
    }
    return r_string;
}
