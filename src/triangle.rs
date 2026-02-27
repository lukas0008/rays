use crate::{plane::Plane, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

#[derive(Debug, Clone)]
pub struct FatTriangle {
    pub triangle: Triangle,
    pub normal: Vec3,
    pub v01_normal: Vec3,
    pub v12_normal: Vec3,
    pub v20_normal: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Self { v0, v1, v2 }
    }
}

impl From<Triangle> for FatTriangle {
    fn from(triangle: Triangle) -> Self {
        let v0v1 = triangle.v1 - triangle.v0;
        let v0v2 = triangle.v2 - triangle.v0;
        let normal = -v0v1.cross(&v0v2).normalize();
        let v01_normal = v0v1.cross(&normal).normalize();
        let v12_normal = (triangle.v2 - triangle.v1).cross(&normal).normalize();
        let v20_normal = (triangle.v0 - triangle.v2).cross(&normal).normalize();
        Self {
            triangle,
            normal,
            v01_normal,
            v12_normal,
            v20_normal,
        }
    }
}

impl From<Triangle> for Plane {
    fn from(triangle: Triangle) -> Self {
        let v0v1 = triangle.v1 - triangle.v0;
        let v0v2 = triangle.v2 - triangle.v0;
        let normal = v0v1.cross(&v0v2).normalize();
        Self {
            normal,
            point: triangle.v0,
        }
    }
}

impl FatTriangle {
    pub fn intersect(&self, ray: &Ray) -> Option<Vec3> {
        let plane: Plane = self.triangle.clone().into();
        let intersection = plane.intersect(ray)?;

        if (intersection - self.triangle.v0).dot(&self.v01_normal) >= 0.0
            && (intersection - self.triangle.v1).dot(&self.v12_normal) >= 0.0
            && (intersection - self.triangle.v2).dot(&self.v20_normal) >= 0.0
        {
            Some(intersection)
        } else {
            None
        }
    }
}
