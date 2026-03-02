use std::sync::mpsc::SyncSender;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{HEIGHT, WIDTH, camera::Camera, model::Model, render::FrameUpdate, vec2::Vec2};

#[derive(Clone, Debug, Default)]
pub struct Scene {
    pub models: Vec<Model>,
    pub camera: Camera,
}

pub fn paint_scene(scene: Scene, frame_sender: SyncSender<FrameUpdate>) {
    let triangles = scene
        .models
        .iter()
        .flat_map(|m| m.triangles())
        .collect::<Vec<_>>();
    for x in 0..WIDTH {
        (0..HEIGHT).into_par_iter().for_each(|y| {
            let ray = scene.camera.get_ray(
                Vec2 {
                    x: WIDTH as f32 / HEIGHT as f32,
                    y: 1.,
                },
                Vec2 {
                    x: (x as f32 / HEIGHT as f32) * 2. - 1.,
                    y: 1. - (y as f32 / WIDTH as f32) * 2.,
                },
            );
            let intersect = triangles
                .iter()
                .filter_map(|t| t.intersect(&ray).map(|v| (t, v)))
                .reduce(|a, b| {
                    if (a.1 - scene.camera.position).len_squared()
                        < (b.1 - scene.camera.position).len_squared()
                    {
                        a
                    } else {
                        b
                    }
                });
            // intersect.sort_by(|(_, i1), (_, i2)| {
            //     (*i1 - scene.camera.position)
            //         .len_squared()
            //         .total_cmp(&(*i2 - scene.camera.position).len_squared())
            // });
            // flip y because pixels are stored from top to bottom
            let y = HEIGHT - y - 1;
            let idx = ((y * 800 + x) * 4) as usize;
            if let Some((t, _v)) = intersect {
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
        });
    }
}
