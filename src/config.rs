use std::collections::HashMap;

use raylib::ffi::KeyboardKey;

use crate::event;

pub struct Config {
    keybindings: HashMap<KeyboardKey, event::Event>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            keybindings: HashMap::new(),
        }
    }

    pub fn keybind(&self, key: KeyboardKey) -> Option<&event::Event> {
        self.keybindings.get(&key)
    }
}

pub static CONFIG: Config = Config {
    keybindings: HashMap::new(),
};
