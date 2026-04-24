use std::sync::{Arc, atomic::AtomicBool, mpsc::SyncSender};

use itertools::Itertools;
use rand::seq::SliceRandom;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use crate::{HEIGHT, WIDTH, camera::Camera, model::Model, render::FrameUpdate, vec2::Vec2};

#[derive(Clone, Debug, Default)]
pub struct Scene {
    pub models: Vec<Model>,
    pub camera: Camera,
}

pub fn paint_scene(scene: Scene, frame_sender: SyncSender<FrameUpdate>, running: Arc<AtomicBool>) {
    let start = std::time::Instant::now();
    let triangles = scene
        .models
        .iter()
        .flat_map(|m| m.triangles())
        .collect::<Vec<_>>();

    let mut pixels = (0..WIDTH).cartesian_product(0..HEIGHT).collect::<Vec<_>>();
    pixels.shuffle(&mut rand::rng());
    pixels.into_par_iter().for_each(|(x, y)| {
        if !running.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }
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
    println!("Scene painted in {}ms", start.elapsed().as_millis());
    // }
}
