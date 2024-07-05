#![windows_subsystem = "windows"]

use std::sync::{Arc, Mutex, OnceLock};
use winit::event_loop::{ControlFlow, EventLoop};

mod app_state;
use app_state::AppState;

mod app;
use app::App;

mod make_bind;
use make_bind::make_binds;

mod defaults;

mod load_config;
use load_config::load_config;

static INPUTS: OnceLock<Arc<Mutex<u8>>> = OnceLock::new();

fn main() {
    // contains a buffer of inputs from the input thread to the game thread
    INPUTS.get_or_init(|| Arc::new(Mutex::new(0)));

    simple_logging::log_to_file("debug.log", log::LevelFilter::Info).unwrap();

    // Starts the gameloop and spawns the game window
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let (triggers, blind_len) = load_config("config.txt");
    log::info!("Triggers: {:?}", triggers);
    make_binds(INPUTS.get().unwrap(), triggers.as_slice());

    let mut app = App {
        window: None,
        app_state: AppState::new(blind_len),
        pixels: None,
        inputs: INPUTS.get().unwrap().clone(),
    };
    event_loop.run_app(&mut app).unwrap();
}
