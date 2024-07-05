#![windows_subsystem = "windows"]

use std::fs::read_to_string;
use std::str::FromStr;
use std::sync::{Arc, Mutex, OnceLock};

use log::warn;

use winit::event_loop::{ControlFlow, EventLoop};

mod app_state;
use app_state::AppState;

mod app;
use app::App;

mod make_bind;
use make_bind::make_binds;

static INPUTS: OnceLock<Arc<Mutex<u8>>> = OnceLock::new();

fn main() {
    // contains a buffer of inputs from the input thread to the game thread
    INPUTS.get_or_init(|| Arc::new(Mutex::new(0)));

    simple_logging::log_to_file("debug.log", log::LevelFilter::Info).unwrap();

    // Starts the gameloop and spawns the game window
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    // Loads the configuration. Returns a default configuration if it fails at any point
    // Default configuration: Empty set of triggers, 1 second dark
    let (triggers, blind_len) = {
        let default_triggers: Vec<String> = Vec::from([
            "w".into(),
            "a".into(),
            "s".into(),
            "d".into(),
            "leftbutton".into(),
            "rightbutton".into(),
            "rightbutton".into(),
        ]);
        match read_to_string("config.txt") {
            Ok(s) => {
                let lines: Vec<String> = s.split("\n").map(|x| x.trim().into()).collect();
                let blind_len = {
                    if lines.len() >= 2 {
                        match FromStr::from_str(&lines[1]) {
                            Ok(f) => f,
                            Err(_) => 1.0,
                        }
                    } else {
                        1.0
                    }
                };
                let blinders = {
                    if lines.len() >= 1 {
                        lines[0]
                            .split(",")
                            .map(|x| x.trim().to_lowercase().into())
                            .collect::<Vec<String>>()
                    } else {
                        default_triggers
                    }
                };
                (blinders, blind_len)
            }
            Err(_) => {
                warn!("No debug.log found. Using default config.");
                (default_triggers, 1.0)
            }
        }
    };
    make_binds(INPUTS.get().unwrap(), triggers.as_slice());

    let mut app = App {
        window: None,
        app_state: AppState::new(blind_len),
        pixels: None,
        inputs: INPUTS.get().unwrap().clone(),
    };
    event_loop.run_app(&mut app).unwrap();
}
