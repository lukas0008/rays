use crate::{
    color::Color,
    model::Model,
    obj::parse_obj,
    render::{FrameUpdate, Renderer},
    triangle::{FatTriangle, Triangle},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use winit::event_loop::EventLoop;

mod camera;
mod color;
mod model;
mod obj;
mod plane;
mod quat;
mod ray;
mod render;
mod triangle;
mod vec2;
mod vec3;
mod vertex;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
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
    let camera = camera::Camera::new(
        (0.0, 1.0, -5.0).into(),
        (0.0, 0.0, 1.0).into(),
        (0.0, 1.0, 0.0).into(),
        90.0,
    );
    let monki = parse_obj(include_str!("./monki.obj")).unwrap();
    let model = Model::new(vec![triangle, triangle2]);
    let models = [model, monki];
    let (mut renderer, frame_sender) = Renderer::new((WIDTH, HEIGHT));
    frame_sender.send(FrameUpdate::Clear(0.into())).unwrap();
    std::thread::spawn(move || {
        let triangles = models
            .iter()
            .flat_map(|m| m.triangles())
            .collect::<Vec<_>>();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let ray = camera.get_ray(
                    vec2::Vec2 {
                        x: WIDTH as f32 / HEIGHT as f32,
                        y: 1.,
                    },
                    vec2::Vec2 {
                        x: (x as f32 / HEIGHT as f32) * 2. - 1.,
                        y: (y as f32 / WIDTH as f32) * 2. - 1.,
                    },
                );
                let mut intersect = triangles
                    .iter()
                    .filter_map(|t| t.intersect(&ray).map(|v| (t, v)))
                    .collect::<Vec<_>>();
                intersect.sort_by(|(_, i1), (_, i2)| {
                    (*i1 - camera.position)
                        .len_squared()
                        .total_cmp(&(*i2 - camera.position).len_squared())
                });
                // flip y because pixels are stored from top to bottom
                let y = HEIGHT - y - 1;
                let idx = ((y * 800 + x) * 4) as usize;
                if let Some((t, _v)) = intersect.first() {
                    let _ = frame_sender.send(FrameUpdate::Individual {
                        idx,
                        color: t.triangle.color,
                    });
                } else {
                    let _ = frame_sender.send(FrameUpdate::Individual {
                        idx,
                        color: 0x444444.into(),
                    });
                }
            }
        }
        dbg!("done");
    });
    eloop.run_app(&mut renderer).unwrap();
}
