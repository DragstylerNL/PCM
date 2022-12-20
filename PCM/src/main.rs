use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;
use std::process::Command;
use std::fmt::Write;
use toml;
use serde_derive::Deserialize;
use std::fs;
use std::process::exit;

const COMMAND_KEY: &str = "PCM";

#[derive(Deserialize)]
struct Commands {
    command: String,
    path: String,
}

fn main() -> anyhow::Result <()> {
    
    let device_state = DeviceState::new();
    let mut old_key = device_state.get_keys();
    let mut input = String::new();

    let contents = match fs::read_to_string("datafiles/cmd_and_path.toml") {
        Ok(c) => c,
        Err(_) => {
            exit(1);
        }
    };
    let data_table: HashMap<String, Vec<Commands>> = toml::from_str(&contents).unwrap();
    let data:&[Commands] = &data_table["commands"];

    // MAIN LOOP =======
    loop {
        read_input(&device_state, &mut old_key, &mut input);

        if input.contains(COMMAND_KEY) {
            
            read_lauchables(&data, &mut input);
            
        }
    }
}

fn read_input(device_state:&DeviceState, old_key:&mut Vec<Keycode>, input:&mut String) {
    let keys = device_state.get_keys();
    if *old_key != keys {
        for key in keys.iter(){
            match key {
                Keycode::Enter | Keycode::Space | Keycode::LShift => { *input = String::new(); },
                _ => {
                    write!(*input, "{:?}", key);
                }
            }
        }
        *old_key = keys;
    }
}

fn read_lauchables(data:&[Commands], input:&mut String){
    for i in 0..data.len() {
        if input.contains(&data[i].command) {
            let _ = Command::new(&data[i].path).spawn();
            *input = String::new();
        }
    }
}
