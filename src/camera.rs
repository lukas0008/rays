use crate::{vec2::Vec2, vec3::Vec3};

pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: Vec3, forward: Vec3, up: Vec3, fov: f32) -> Self {
        let right = -forward.cross(&up).normalize();
        let up = right.cross(&forward).normalize();
        Self {
            position,
            forward: forward.normalize(),
            up,
            right,
            fov: fov.to_radians(),
        }
    }
    pub fn get_ray(&self, size: Vec2, Vec2 { x, y }: Vec2) -> crate::ray::Ray {
        let aspect_ratio = size.x / size.y;
        let plane_size = (self.fov / 2.).atan() / aspect_ratio;
        let plane_center = self.position + self.forward;
        let vector_point =
            plane_center + x * self.right * aspect_ratio * plane_size + y * self.up * plane_size;

        let ray_direction = (vector_point - self.position).normalize();

        crate::ray::Ray {
            origin: self.position,
            direction: ray_direction.normalize(),
        }
    }
}
