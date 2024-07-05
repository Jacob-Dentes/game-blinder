use std::sync::{Arc, Mutex};
use std::thread;

use log::warn;

use inputbot::{get_keybd_key, KeybdKey, MouseButton};

fn make_char_keybind(inputs: &'static Arc<Mutex<u8>>, s: &str) {
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

fn get_keybd_longkey(s: &str) -> Option<KeybdKey> {
    match s.to_lowercase().as_str() {
        "backspace" => Some(KeybdKey::BackspaceKey),
        "tab" => Some(KeybdKey::TabKey),
        "enter" => Some(KeybdKey::EnterKey),
        "escape" => Some(KeybdKey::EscapeKey),
        "space" => Some(KeybdKey::SpaceKey),
        "pageup" => Some(KeybdKey::PageUpKey),
        "pagedown" => Some(KeybdKey::PageDownKey),
        "end" => Some(KeybdKey::EndKey),
        "home" => Some(KeybdKey::HomeKey),
        "left" => Some(KeybdKey::LeftKey),
        "up" => Some(KeybdKey::UpKey),
        "right" => Some(KeybdKey::RightKey),
        "down" => Some(KeybdKey::DownKey),
        "insert" => Some(KeybdKey::InsertKey),
        "delete" => Some(KeybdKey::DeleteKey),
        "lsuper" | "lwindows" | "lcommand" => Some(KeybdKey::LSuper),
        "rsuper" | "rwindows" | "rcommand" => Some(KeybdKey::RSuper),
        "numpad0" => Some(KeybdKey::Numpad0Key),
        "numpad1" => Some(KeybdKey::Numpad1Key),
        "numpad2" => Some(KeybdKey::Numpad2Key),
        "numpad3" => Some(KeybdKey::Numpad3Key),
        "numpad4" => Some(KeybdKey::Numpad4Key),
        "numpad5" => Some(KeybdKey::Numpad5Key),
        "numpad6" => Some(KeybdKey::Numpad6Key),
        "numpad7" => Some(KeybdKey::Numpad7Key),
        "numpad8" => Some(KeybdKey::Numpad8Key),
        "numpad9" => Some(KeybdKey::Numpad9Key),
        "f1" => Some(KeybdKey::F1Key),
        "f2" => Some(KeybdKey::F2Key),
        "f3" => Some(KeybdKey::F3Key),
        "f4" => Some(KeybdKey::F4Key),
        "f5" => Some(KeybdKey::F5Key),
        "f6" => Some(KeybdKey::F6Key),
        "f7" => Some(KeybdKey::F7Key),
        "f8" => Some(KeybdKey::F8Key),
        "f9" => Some(KeybdKey::F9Key),
        "f10" => Some(KeybdKey::F10Key),
        "f11" => Some(KeybdKey::F11Key),
        "f12" => Some(KeybdKey::F12Key),
        "f13" => Some(KeybdKey::F13Key),
        "f14" => Some(KeybdKey::F14Key),
        "f15" => Some(KeybdKey::F15Key),
        "f16" => Some(KeybdKey::F16Key),
        "f17" => Some(KeybdKey::F17Key),
        "f18" => Some(KeybdKey::F18Key),
        "f19" => Some(KeybdKey::F19Key),
        "f20" => Some(KeybdKey::F20Key),
        "f21" => Some(KeybdKey::F21Key),
        "f22" => Some(KeybdKey::F22Key),
        "f23" => Some(KeybdKey::F23Key),
        "f24" => Some(KeybdKey::F24Key),
        "numlock" => Some(KeybdKey::NumLockKey),
        "scrolllock" => Some(KeybdKey::ScrollLockKey),
        "capslock" => Some(KeybdKey::CapsLockKey),
        "leftshift" => Some(KeybdKey::LShiftKey),
        "rightshift" => Some(KeybdKey::RShiftKey),
        "leftcontrol" => Some(KeybdKey::LControlKey),
        "rightcontrol" => Some(KeybdKey::RControlKey),
        "leftalt" => Some(KeybdKey::LAltKey),
        "rightalt" => Some(KeybdKey::RAltKey),
        "back" => Some(KeybdKey::BrowserBackKey),
        "forward" => Some(KeybdKey::BrowserForwardKey),
        "refresh" => Some(KeybdKey::BrowserRefreshKey),
        "volumemute" => Some(KeybdKey::VolumeMuteKey),
        "volumedown" => Some(KeybdKey::VolumeDownKey),
        "volumeup" => Some(KeybdKey::VolumeUpKey),
        "medianext" => Some(KeybdKey::MediaNextTrackKey),
        "mediaprevious" => Some(KeybdKey::MediaPrevTrackKey),
        "mediastop" => Some(KeybdKey::MediaStopKey),
        "mediaplay" => Some(KeybdKey::MediaPlayPauseKey),
        "backquote" => Some(KeybdKey::BackquoteKey),
        "slash" => Some(KeybdKey::SlashKey),
        "backslash" => Some(KeybdKey::BackslashKey),
        "comma" => Some(KeybdKey::CommaKey),
        "period" => Some(KeybdKey::PeriodKey),
        "minus" => Some(KeybdKey::MinusKey),
        "quotekey" => Some(KeybdKey::QuoteKey),
        "semicolon" => Some(KeybdKey::SemicolonKey),
        "leftbracket" => Some(KeybdKey::LBracketKey),
        "rightbracket" => Some(KeybdKey::RBracketKey),
        "equal" => Some(KeybdKey::EqualKey),
        _ => None,
    }
}

fn make_mousebind(inputs: &'static Arc<Mutex<u8>>, s: &str) -> bool {
    match get_mousebd_button(s) {
        Some(mse_button) => {
            mse_button.bind(|| {
                let mut num = inputs.lock().unwrap();
                *num = 1;
            });
            true
        }
        None => false,
    }
}

fn make_longkeybind(inputs: &'static Arc<Mutex<u8>>, s: &str) -> bool {
    match get_keybd_longkey(s) {
        Some(kbd_key) => {
            kbd_key.bind(|| {
                let mut num = inputs.lock().unwrap();
                *num = 1;
            });
            true
        }
        None => false,
    }
}

pub fn make_binds(inputs: &'static Arc<Mutex<u8>>, triggers: &[String]) {
    for s in triggers {
        match s.len() {
            1 => make_char_keybind(inputs, s),
            2.. => {
                if !(make_mousebind(inputs, s) || make_longkeybind(inputs, s)) {
                    warn!("Unknown bind: {}", s);
                }
            }
            0 => warn!("Make binds given empty string. This is a bug."),
        }
    }
    thread::spawn(|| {
        inputbot::handle_input_events();
    });
}
