use device_query::{DeviceQuery, DeviceState};
use std::collections::HashMap;
use std::fs;
use toml;

mod input_reader;
mod command_managers;

fn main() -> anyhow::Result<()> {
    // INIT SETUP
    let device_state = DeviceState::new();

    let mut keyboard: input_reader::Input = input_reader::Input {
        input: String::new(),
        old_key: device_state.get_keys(),
    };

    let contents = fs::read_to_string("datafiles/cmd_and_path.toml")?;
    let data_table: HashMap<String, Vec<command_managers::Commands>> = toml::from_str(&contents)?;
    let data: &[command_managers::Commands] = &data_table["commands"];

    // MAIN LOOP
    loop {
        keyboard = input_reader::read_input(&device_state, &keyboard);
        keyboard.input = command_managers::check_commandword(&data, &keyboard.input);
    }
}
