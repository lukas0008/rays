use std::ops::{Add, Mul};

use crate::{color::Color, plane::Plane, quat::Quat, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub color: Color,
    pub opacity: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct FatTriangle {
    pub triangle: Triangle,
    pub normal: Vec3,
    pub v01_normal: Vec3,
    pub v12_normal: Vec3,
    pub v20_normal: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, color: Color) -> Self {
        Self {
            v0,
            v1,
            v2,
            color,
            opacity: 255,
        }
    }
    pub fn vertices(&self) -> [Vec3; 3] {
        [self.v0, self.v1, self.v2]
    }
    pub fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }
    pub fn with_opacity(self, opacity: u8) -> Self {
        Self { opacity, ..self }
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
        let plane: Plane = self.triangle.into();
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
    pub fn vertices(&self) -> [Vec3; 3] {
        self.triangle.vertices()
    }
}

impl Mul<Quat> for Triangle {
    type Output = Triangle;
    fn mul(self, rhs: Quat) -> Self::Output {
        Self {
            v0: self.v0 * rhs,
            v1: self.v1 * rhs,
            v2: self.v2 * rhs,
            color: self.color,
            opacity: self.opacity,
        }
    }
}
impl Add<Vec3> for Triangle {
    type Output = Triangle;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            v0: self.v0 + rhs,
            v1: self.v1 + rhs,
            v2: self.v2 + rhs,
            color: self.color,
            opacity: self.opacity,
        }
    }
}
impl Add<Triangle> for Vec3 {
    type Output = Triangle;
    fn add(self, rhs: Triangle) -> Self::Output {
        rhs + self
    }
}

impl Mul<Triangle> for Quat {
    type Output = Triangle;
    fn mul(self, rhs: Triangle) -> Self::Output {
        rhs * self
    }
}

impl Mul<Quat> for FatTriangle {
    type Output = FatTriangle;
    fn mul(self, rhs: Quat) -> Self::Output {
        Self {
            triangle: self.triangle * rhs,
            normal: self.normal * rhs,
            v01_normal: self.v01_normal * rhs,
            v12_normal: self.v12_normal * rhs,
            v20_normal: self.v20_normal * rhs,
        }
    }
}

impl Mul<FatTriangle> for Quat {
    type Output = FatTriangle;
    fn mul(self, rhs: FatTriangle) -> Self::Output {
        rhs * self
    }
}

impl Add<Vec3> for FatTriangle {
    type Output = FatTriangle;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            triangle: self.triangle + rhs,
            normal: self.normal,
            v01_normal: self.v01_normal,
            v12_normal: self.v12_normal,
            v20_normal: self.v20_normal,
        }
    }
}
impl Add<FatTriangle> for Vec3 {
    type Output = FatTriangle;
    fn add(self, rhs: FatTriangle) -> Self::Output {
        rhs + self
    }
}
