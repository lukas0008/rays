use crate::{triangle::Triangle, vec3::Vec3};

pub struct Model {
    pub triangles: Vec<Triangle>,
    pub transform: [[f32; 4]; 4],
    pub translate: Vec3
}
