#![windows_subsystem = "windows"]
use pixels::wgpu::Backends;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use inputbot::{KeybdKey, MouseButton};

use std::collections::{BTreeSet, VecDeque};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use log::info;
use std::fs::read_to_string;
use std::str::FromStr;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

static INPUTS: OnceLock<Arc<Mutex<VecDeque<String>>>> = OnceLock::new();

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    last_input: Instant,
    dark_duration: Duration,
    triggers: BTreeSet<String>,
    debugging: bool,
}

struct App {
    window: Option<Window>,
    world: World,
    pixels: Option<Pixels>,
    inputs: Arc<Mutex<VecDeque<String>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open("assets/ph--eye-slash-light.png")
                .expect("Failed to load window icon.")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        let icon = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height)
            .expect("Failed to process icon file.");
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_transparent(true)
                    .with_decorations(false)
                    .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
                    .with_window_icon(Some(icon.clone()))
                    .with_title("Blinder"),
            )
            .unwrap();
        window.set_cursor_hittest(false).unwrap();
        let pixels = {
            let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
            PixelsBuilder::new(WIDTH, HEIGHT, surface_texture)
                .clear_color(pixels::wgpu::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                })
                .blend_state(pixels::wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING)
                .wgpu_backend(Backends::GL)
                .build()
                .unwrap()
        };
        self.pixels = Some(pixels);
        self.window = Some(window);
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                match self.pixels.as_mut() {
                    None => (),
                    Some(p) => {
                        self.world.draw(p.frame_mut());
                        p.render().unwrap();
                    }
                };
                match self.inputs.lock() {
                    Ok(mut deq) => {
                        for s in deq.drain(..) {
                            self.world.update(s);
                        }
                    }
                    Err(_) => (),
                }
                self.window
                    .as_ref()
                    .unwrap()
                    .set_window_level(winit::window::WindowLevel::AlwaysOnTop);
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(size) => match self.pixels.as_mut() {
                None => (),
                Some(pixels) => pixels.resize_surface(size.width, size.height).unwrap(),
            },
            _ => (),
        }
    }
}

fn main() {
    // contains a buffer of inputs from the input thread to the game thread
    INPUTS.get_or_init(|| Arc::new(Mutex::new(VecDeque::new())));

    {
        // The closure runs each time a key is pressed
        KeybdKey::bind_all(|event| match inputbot::from_keybd_key(event) {
            Some(c) => {
                let mut deq = INPUTS.get().unwrap().lock().unwrap();
                deq.push_back(c.to_string());
            }
            None => println!("Unregistered key"),
        });
        // The closure runs each time a mouse button is pressed
        MouseButton::bind_all(|event| {
            let mut deq = INPUTS.get().unwrap().lock().unwrap();
            deq.push_back(format!("{:?}", event).to_string());
        });
    }
    // Starts the thread that listens for keyboard and mouse events
    thread::spawn(|| {
        inputbot::handle_input_events();
    });
    // Starts the gameloop and spawns the game window
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let (triggers, blind_len, debugging) = {
        match read_to_string("config.txt") {
            Ok(s) => {
                let lines: Vec<String> = s.split("\n").map(|x| x.trim().into()).collect();
                let debugging = {
                    if lines.len() >= 3 {
                        match FromStr::from_str(&lines[2]) {
                            Ok(b) => b,
                            Err(_) => false,
                        }
                    } else {
                        false
                    }
                };
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
                            .collect::<BTreeSet<String>>()
                    } else {
                        BTreeSet::new()
                    }
                };
                (blinders, blind_len, debugging)
            }
            Err(_) => (BTreeSet::new(), 1.0, true),
        }
    };
    if debugging {
        simple_logging::log_to_file("debug.log", log::LevelFilter::Info).unwrap();
    }

    let mut app = App {
        window: None,
        world: World::new(triggers, blind_len, debugging),
        pixels: None,
        inputs: INPUTS.get().unwrap().clone(),
    };
    event_loop.run_app(&mut app).unwrap();
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new(triggers: BTreeSet<String>, duration: f32, debugging: bool) -> Self {
        Self {
            last_input: Instant::now(),
            triggers,
            dark_duration: Duration::from_secs_f32(duration),
            debugging,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, s: String) {
        if self.triggers.contains(&(s.to_lowercase())) {
            self.last_input = Instant::now();
        }
        if self.debugging {
            info!("{}", s);
        }
    }

    /// Draw the `World` state to the frame buffer.
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        let rgba = {
            if self.last_input.elapsed() > self.dark_duration {
                [0x00, 0x00, 0x00, 0x00]
            } else {
                [0x00, 0x00, 0x00, 0xff]
            }
        };
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&rgba);
        }
    }
}
