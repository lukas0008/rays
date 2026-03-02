use std::ops::{Mul, MulAssign};

use crate::{quat::Quat, vec2::Vec2, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub fov: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0., 0., 0.),
            forward: Vec3::new(0., 0., -1.),
            up: Vec3::new(0., 1., 0.),
            right: Vec3::new(1., 0., 0.),
            fov: 90f32.to_radians(),
        }
    }
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

        let ray_direction = vector_point - self.position;

        crate::ray::Ray {
            origin: self.position,
            direction: ray_direction,
        }
    }
}

impl Mul<Quat> for Camera {
    type Output = Self;

    fn mul(self, quat: Quat) -> Self {
        let forward = quat * self.forward;
        let up = quat * self.up;
        let right = quat * self.right;
        Self {
            position: self.position,
            forward,
            up,
            right,
            fov: self.fov,
        }
    }
}

impl Mul<Camera> for Quat {
    type Output = Camera;

    fn mul(self, cam: Camera) -> Camera {
        cam * self
    }
}

impl Mul<Quat> for &Camera {
    type Output = Camera;

    fn mul(self, quat: Quat) -> Camera {
        self.clone() * quat
    }
}

impl Mul<&Camera> for Quat {
    type Output = Camera;

    fn mul(self, cam: &Camera) -> Camera {
        cam.clone() * self
    }
}

impl MulAssign<Quat> for Camera {
    fn mul_assign(&mut self, quat: Quat) {
        *self = &*self * quat;
    }
}
