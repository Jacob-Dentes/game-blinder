use std::collections::BTreeSet;
use std::time::{Duration, Instant};

use log::info;

pub struct AppState {
    last_input: Instant,
    dark_duration: Duration,
    triggers: BTreeSet<String>,
    debugging: bool,
}

impl AppState {
    pub fn new(triggers: BTreeSet<String>, duration: f32, debugging: bool) -> Self {
        Self {
            last_input: Instant::now(),
            triggers,
            dark_duration: Duration::from_secs_f32(duration),
            debugging,
        }
    }

    pub fn update(&mut self, s: String) {
        if self.triggers.contains(&(s.to_lowercase())) {
            self.last_input = Instant::now();
        }
        if self.debugging {
            info!("{}", s);
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
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
