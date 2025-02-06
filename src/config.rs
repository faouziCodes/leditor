use std::collections::HashMap;

use raylib::ffi::KeyboardKey;

use crate::event;

pub struct Config {
    keybindings: HashMap<KeyboardKey, event::Event>,
}
