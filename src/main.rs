use pixels::wgpu::Backends;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

struct App {
    window: Option<Window>,
    world: World,
    pixels: Option<Pixels>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_transparent(true)
                    .with_decorations(false)
                    .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None))),
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
                self.world.update();
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
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App {
        window: None,
        world: World::new(),
        pixels: None,
    };
    event_loop.run_app(&mut app).unwrap();
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 10,
            velocity_y: 10,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x00, 0x00, 0x00, 0x00]
            };
            pixel.copy_from_slice(&rgba);
        }
    }
}
