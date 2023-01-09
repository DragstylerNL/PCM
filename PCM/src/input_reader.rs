use device_query::{DeviceQuery, DeviceState, Keycode};
use std::fmt::Write;

pub struct Input {
    pub input: String,
    pub old_key: Vec<Keycode>,
}

pub fn read_input(device_state: &DeviceState, keyboard: &Input) -> Input {
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
