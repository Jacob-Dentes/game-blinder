use std::time::{Duration, Instant};

pub struct AppState {
    last_input: Instant,
    dark_duration: Duration,
}

impl AppState {
    pub fn new(duration: f32) -> Self {
        Self {
            last_input: Instant::now(),
            dark_duration: Duration::from_secs_f32(duration),
        }
    }

    pub fn update(&mut self, n: &u8) {
        if n >= &1 {
            self.last_input = Instant::now();
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
