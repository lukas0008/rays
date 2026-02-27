use crate::{ray::Ray, vec3::Vec3};

pub struct Plane {
    pub normal: Vec3,
    pub point: Vec3,
}

impl Plane {
    pub fn intersect(&self, ray: &Ray) -> Option<Vec3> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < 1e-6 {
            return None; // Ray is parallel to the plane
        }

        let t = (self.point - ray.origin).dot(&self.normal) / denom;
        if t < 0.0 {
            return None; // Intersection is behind the ray origin
        }

        Some(ray.origin + ray.direction * t)
    }
}
