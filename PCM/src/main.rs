use device_query::{DeviceQuery, DeviceState, Keycode};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::process::Command;
use toml;

const COMMAND_KEY: &str = "PCM";

#[derive(Deserialize)]
struct Commands {
    command: String,
    path: String,
}

struct Input {
    input: String,
    old_key: Vec<Keycode>,
}

fn main() -> anyhow::Result<()> {
    let device_state = DeviceState::new();

    let mut keyboard: Input = Input {
        input: String::new(),
        old_key: device_state.get_keys(),
    };

    let contents = fs::read_to_string("datafiles/cmd_and_path.toml")?;
    let data_table: HashMap<String, Vec<Commands>> = toml::from_str(&contents)?;
    let data: &[Commands] = &data_table["commands"];

    // MAIN LOOP =======
    loop {
        keyboard = read_input(&device_state, &keyboard);

        if keyboard.input.contains(COMMAND_KEY) {
            keyboard.input = read_lauchables(&data, &keyboard.input);
        }
    }
}

fn read_input(device_state: &DeviceState, keyboard: &Input) -> Input {
    let mut current_keyboard: Input = Input {
        input: keyboard.input.to_string(),
        old_key: device_state.get_keys(),
    };
    if keyboard.old_key != current_keyboard.old_key {
        for key in current_keyboard.old_key.iter() {
            match key {
                Keycode::Enter | Keycode::Space | Keycode::LShift => {
                    current_keyboard.input = String::new();
                }
                _ => {
                    write!(current_keyboard.input, "{:?}", key);
                }
            }
        }
    }
    return current_keyboard;
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
