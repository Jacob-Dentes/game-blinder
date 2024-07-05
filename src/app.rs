use std::sync::{Arc, Mutex};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use pixels::{wgpu::Backends, Pixels, PixelsBuilder, SurfaceTexture};

use crate::AppState;

pub struct App {
    pub window: Option<Window>,
    pub app_state: AppState,
    pub pixels: Option<Pixels>,
    pub inputs: Arc<Mutex<u8>>,
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
        let size = window.inner_size();
        let pixels = {
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            PixelsBuilder::new(size.width, size.height, surface_texture)
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
                        self.app_state.draw(p.frame_mut());
                        p.render().unwrap();
                    }
                };
                match self.inputs.lock() {
                    Ok(mut num) => {
                        self.app_state.update(&num);
                        *num = 0;
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
