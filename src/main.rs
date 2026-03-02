#![feature(portable_simd)]

use std::sync::{Arc, atomic::AtomicBool, mpsc::SyncSender};

use crate::{
    color::Color,
    model::Model,
    obj::parse_obj,
    quat::Quat,
    render::{FrameUpdate, Renderer},
    scene::{Scene, paint_scene},
    triangle::{FatTriangle, Triangle},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use winit::{
    event::WindowEvent,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

mod camera;
mod color;
mod model;
mod obj;
mod plane;
mod quat;
mod ray;
mod render;
mod scene;
mod triangle;
mod vec2;
mod vec3;
mod vertex;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
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
        (3.0, -3.0, 0.0).into(),
        Color(0xFFFF0000),
    )
    .into();
    let mut camera = camera::Camera::new(
        (0.0, 1.0, -5.0).into(),
        (0.0, 0.0, 1.0).into(),
        (0.0, 1.0, 0.0).into(),
        90.0,
    );
    let monki = parse_obj(include_str!("./monki.obj")).unwrap();
    let model = Model::new(vec![triangle, triangle2]);
    let models = vec![model, monki];
    #[cfg(feature = "stuff")]
    {
        use std::sync::{Arc, atomic::AtomicBool};
        let (sender, receiver) = std::sync::mpsc::sync_channel(1024);
        let scene = Scene {
            models,
            camera: camera.clone(),
        };
        let a = Arc::new(AtomicBool::new(true));
        let b = a.clone();
        std::thread::spawn(move || {
            while receiver.recv().is_ok() && a.load(std::sync::atomic::Ordering::Relaxed) {}
        });
        paint_scene(scene, sender);
        b.store(false, std::sync::atomic::Ordering::Relaxed);
        return;
    }
    let (mut renderer, frame_sender) = Renderer::new((WIDTH, HEIGHT));
    frame_sender.send(FrameUpdate::Clear(0.into())).unwrap();
    let mut current_running = Arc::new(AtomicBool::new(true));
    fn reset_running(current_running: &mut Arc<AtomicBool>) {
        current_running.store(false, std::sync::atomic::Ordering::Relaxed);
        *current_running = Arc::new(AtomicBool::new(true));
    }
    macro_rules! run_new {
        () => {{
            reset_running(&mut current_running);
            let current_running = current_running.clone();
            let scene = Scene {
                models: models.clone(),
                camera: camera.clone(),
            };
            let frame_sender = frame_sender.clone();
            std::thread::spawn(move || {
                paint_scene(scene, frame_sender, current_running);
                dbg!("done");
            });
        }};
    };
    renderer.set_on_event(move |ev| {
        if let WindowEvent::KeyboardInput {
            device_id: _,
            event,
            is_synthetic: _,
        } = ev
        {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::Enter) if event.state.is_pressed() => {
                    run_new!();
                }
                PhysicalKey::Code(KeyCode::KeyD) if event.state.is_pressed() => {
                    camera.position.x += 1.;
                    run_new!();
                }
                PhysicalKey::Code(KeyCode::KeyA) if event.state.is_pressed() => {
                    camera.position.x -= 1.;
                    run_new!();
                }
                PhysicalKey::Code(KeyCode::KeyW) if event.state.is_pressed() => {
                    camera.position.z += 1.;
                    run_new!();
                }
                PhysicalKey::Code(KeyCode::KeyS) if event.state.is_pressed() => {
                    camera.position.z -= 1.;
                    run_new!();
                }
                PhysicalKey::Code(KeyCode::Space) if event.state.is_pressed() => {
                    camera.position.y += 1.;
                    run_new!();
                }
                PhysicalKey::Code(KeyCode::ShiftLeft) if event.state.is_pressed() => {
                    camera.position.y -= 1.;
                    run_new!();
                }
                PhysicalKey::Code(KeyCode::ArrowLeft) if event.state.is_pressed() => {
                    camera *= Quat::from_euler_angles(0., -0.1, 0.);
                    run_new!();
                }
                PhysicalKey::Code(KeyCode::ArrowRight) if event.state.is_pressed() => {
                    camera *= Quat::from_euler_angles(0., 0.1, 0.);
                    run_new!();
                }
                PhysicalKey::Code(KeyCode::ArrowUp) if event.state.is_pressed() => {
                    camera *= Quat::from_euler_angles(-0.1, 0., 0.);
                    run_new!();
                }
                PhysicalKey::Code(KeyCode::ArrowDown) if event.state.is_pressed() => {
                    camera *= Quat::from_euler_angles(0.1, 0., 0.);
                    run_new!();
                }
                _ => {}
            }
        }
    });
    eloop.run_app(&mut renderer).unwrap();
}
