use crate::{
    quat::Quat,
    triangle::{FatTriangle, Triangle},
    vec3::Vec3,
};

#[derive(Clone, Debug)]
pub struct Model {
    pub triangles: Vec<FatTriangle>,
    pub rotation: Quat,
    pub transform: Vec3,
    pub center: Vec3,
    pub radius: f32,
}

fn approximate_sphere(triangles: &[FatTriangle]) -> (Vec3, f32) {
    let vertices = triangles
        .iter()
        .flat_map(|t| t.vertices())
        .collect::<Vec<_>>();
    let mut farthest_pair = (vertices[0], vertices[0], 0.);
    for _ in 0..10.min(vertices.len()) {
        let i1 = rand::random_range(0..vertices.len());
        let i2 = rand::random_range(0..vertices.len());
        if i1 == i2 {
            continue;
        }
        let v1 = vertices[i1];
        let v2 = vertices[i2];
        let dist = (v1 - v2).len();
        if dist > farthest_pair.2 {
            farthest_pair = (v1, v2, dist);
        }
    }

    let middle = (farthest_pair.0 + farthest_pair.1) * 0.5;
    let mut radius = 0.0f32;
    for v in vertices {
        radius = radius.max((v - middle).len());
    }
    (middle, radius)
}

impl Model {
    pub fn new(triangles: Vec<FatTriangle>) -> Self {
        let (center, radius) = approximate_sphere(&triangles);
        Self {
            triangles,
            rotation: Quat::identity(),
            transform: (0., 0., 0.).into(),
            center,
            radius,
        }
    }
    pub fn triangles(&self) -> impl Iterator<Item = FatTriangle> {
        self.triangles
            .iter()
            .map(|t| *t * self.rotation + self.transform)
    }
}
