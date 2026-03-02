use std::ops::{Add, Mul, Neg, Sub};

use crate::quat::Quat;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Vec3 { x, y, z } = self;
        Self {
            x: -x,
            y: -y,
            z: -z,
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        (x, y, z).into()
    }
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn len(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn len_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn normalize(&self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    pub fn rotate(&self, quat: Quat) -> Self {
        let q_vec = Vec3 {
            x: quat.x,
            y: quat.y,
            z: quat.z,
        };
        let uv = q_vec.cross(self);
        let uuv = q_vec.cross(&uv);
        *self + (quat.w * uv) + uuv
    }
}

impl Mul<Quat> for Vec3 {
    type Output = Self;

    fn mul(self, quat: Quat) -> Self {
        self.rotate(quat)
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec.rotate(self)
    }
}
