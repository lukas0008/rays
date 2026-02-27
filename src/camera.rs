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
            fov,
        }
    }
    pub fn get_ray(&self, size: Vec2, Vec2 { x, y }: Vec2) -> crate::ray::Ray {
        let aspect_ratio = size.x / size.y;
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let sensor_x = (2.0 * (x + 0.5) / size.x - 1.0) * aspect_ratio * fov_adjustment;
        let sensor_y = (1.0 - 2.0 * (y + 0.5) / size.y) * fov_adjustment;

        let ray_direction = (self.forward + self.right * sensor_x + self.up * sensor_y).normalize();
        crate::ray::Ray {
            origin: self.position,
            direction: ray_direction,
        }
    }
}
