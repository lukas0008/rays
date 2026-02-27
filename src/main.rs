use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event_loop::EventLoop,
    platform::x11::WindowAttributesExtX11,
    window::{Window, WindowAttributes},
};

use crate::{
    camera::Camera,
    triangle::{FatTriangle, Triangle},
};

mod camera;
mod model;
mod plane;
mod quat;
mod ray;
mod triangle;
mod vec2;
mod vec3;

struct Renderer {
    camera: Camera,
    triangle: FatTriangle,
    x: u32,
    y: u32,
    pixels: Option<Pixels<'static>>,
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for Renderer {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_resizable(false)
                    .with_base_size(PhysicalSize::new(800, 600)),
            )
            .expect("Failed to create window");
        let window = Arc::new(window);
        window.set_visible(true);
        self.window = Some(window.clone());
        let size = window.inner_size();

        // Create Pixels AFTER window
        let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());

        let mut pixels = Pixels::new(800, 600, surface_texture).unwrap();
        pixels.set_present_mode(pixels::wgpu::PresentMode::Immediate);

        self.pixels = Some(pixels);
        window.request_redraw();
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let pixels = if let Some(pixels) = &mut self.pixels {
            pixels
        } else {
            return;
        };
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                let pixels_count = 10000;
                for _ in 0..pixels_count {
                    let x = self.x;
                    let y = self.y;
                    if x >= 799 {
                        self.x = 0;
                        self.y += 1;
                    } else {
                        self.x += 1;
                    }
                    if x + y * 800 >= 800 * 600 {
                        break;
                    }
                    let ray = self.camera.get_ray(
                        vec2::Vec2 { x: 800.0, y: 600.0 },
                        vec2::Vec2 {
                            x: x as f32,
                            y: y as f32,
                        },
                    );

                    let intersection = self.triangle.intersect(&ray);
                    // flip y because pixels are stored from top to bottom
                    let y = 599 - y;
                    pixels.frame_mut()[((y * 800 + x) * 4) as usize] = 255;
                    if intersection.is_some() {
                        pixels.frame_mut()[((y * 800 + x) * 4) as usize + 1] = 255;
                    }
                    pixels.frame_mut()[((y * 800 + x) * 4) as usize + 3] = 255;
                }
                pixels.render().unwrap();
                if let Some(window) = &self.window {
                    window.request_redraw();
                } else {
                    println!("something is fucked");
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let eloop = EventLoop::new().unwrap();
    let triangle: FatTriangle = Triangle::new(
        vec3::Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        vec3::Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        vec3::Vec3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        },
    )
    .into();
    let camera = camera::Camera::new(
        vec3::Vec3 {
            x: 0.0,
            y: 1.0,
            z: -5.0,
        },
        vec3::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        vec3::Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        90.0,
    );
    eloop
        .run_app(&mut Renderer {
            camera,
            triangle,
            x: 0,
            y: 0,
            pixels: None,
            window: None,
        })
        .unwrap();
}
