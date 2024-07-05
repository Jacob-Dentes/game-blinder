use std::sync::{Arc, Mutex};
use std::thread;

use log::warn;

use inputbot::{get_keybd_key, MouseButton};

fn make_keybind(inputs: &'static Arc<Mutex<u8>>, s: &str) {
    match s.to_string().chars().next() {
        Some(c) => match get_keybd_key(c) {
            Some(kbd_key) => {
                kbd_key.bind(|| {
                    let mut num = inputs.lock().unwrap();
                    *num = 1;
                });
            }
            None => warn!("Unknown key: {}", c),
        },
        None => warn!("String {} could not be converted to char.", s),
    }
}

fn get_mousebd_button(s: &str) -> Option<MouseButton> {
    match s.to_lowercase().as_str() {
        "leftbutton" => Some(MouseButton::LeftButton),
        "rightbutton" => Some(MouseButton::RightButton),
        "middlebutton" => Some(MouseButton::MiddleButton),
        "x1button" => Some(MouseButton::X1Button),
        "x2button" => Some(MouseButton::X2Button),
        _ => None,
    }
}

fn make_mousebind(inputs: &'static Arc<Mutex<u8>>, s: &str) {
    match get_mousebd_button(s) {
        Some(mse_button) => {
            mse_button.bind(|| {
                let mut num = inputs.lock().unwrap();
                *num = 1;
            });
        }
        None => warn!("Unknown button: {}", s),
    }
}

pub fn make_binds(inputs: &'static Arc<Mutex<u8>>, triggers: &[String]) {
    for s in triggers {
        match s.len() {
            1 => make_keybind(inputs, s),
            2.. => make_mousebind(inputs, s),
            0 => warn!("Make binds given empty string. This is a bug."),
        }
    }
    thread::spawn(|| {
        inputbot::handle_input_events();
    });
}
