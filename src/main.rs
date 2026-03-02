use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture, wgpu::Color as WColor};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event_loop::EventLoop,
    platform::x11::WindowAttributesExtX11,
    window::{Window, WindowAttributes},
};

use crate::{
    camera::Camera,
    color::Color,
    model::Model,
    triangle::{FatTriangle, Triangle},
};

mod camera;
mod color;
mod model;
mod plane;
mod quat;
mod ray;
mod render;
mod triangle;
mod vec2;
mod vec3;
mod vertex;

struct Renderer {
    camera: Camera,
    models: Vec<Model>,
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
                    .with_transparent(false)
                    .with_decorations(false)
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
        pixels.set_present_mode(pixels::wgpu::PresentMode::Fifo);
        pixels.clear_color(WColor {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 0.5,
        });
        dbg!(pixels.render_texture_format());

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
                let triangles = self
                    .models
                    .iter()
                    .flat_map(|m| m.triangles())
                    .collect::<Vec<_>>();
                let pixels_count = 10;
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

                    let intersect = triangles
                        .iter()
                        .find_map(|t| t.intersect(&ray).map(|v| (t, v)));
                    // flip y because pixels are stored from top to bottom
                    let y = 599 - y;
                    let idx = ((y * 800 + x) * 4) as usize;
                    pixels.frame_mut()[idx] = 255;
                    if let Some((t, _v)) = intersect {
                        let c = t.triangle.color;
                        pixels.frame_mut()[idx] = c.r();
                        pixels.frame_mut()[idx + 1] = c.g();
                        pixels.frame_mut()[idx + 2] = c.b();
                        pixels.frame_mut()[idx + 3] = c.a();
                    }
                    // pixels.frame_mut()[((y * 800 + x) * 4) as usize + 3] = 255;
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
        (1.0, 0.0, 0.0).into(),
        (2.0, 0.0, -1.0).into(),
        (2.0, 0.5, 0.0).into(),
        Color(0xFFFFFFFF),
    )
    .into();
    let triangle2: FatTriangle = Triangle::new(
        (2.0, 0.0, 0.0).into(),
        (0.0, 0.5, 0.0).into(),
        (5.0, -5.0, 0.0).into(),
        Color(0xFFFF00FF),
    )
    .into();
    let camera = camera::Camera::new(
        (0.0, 1.0, -5.0).into(),
        (0.0, 0.0, 1.0).into(),
        (0.0, 1.0, 0.0).into(),
        90.0,
    );
    let model = Model::new(vec![triangle, triangle2]);
    eloop
        .run_app(&mut Renderer {
            camera,
            x: 0,
            y: 0,
            models: vec![model],
            pixels: None,
            window: None,
        })
        .unwrap();
}
