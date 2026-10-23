use core::simd;
use std::{
    ops::{Add, Mul, Neg, Sub},
    simd::{Simd, num::SimdFloat},
};

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
        // simd average improvement negligable, but more stable
        (self.simd() + other.simd()).into()
        // Self {
        //     x: self.x + other.x,
        //     y: self.y + other.y,
        //     z: self.z + other.z,
        // }
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

    #[inline]
    fn sub(self, other: Self) -> Self {
        // simd improved perf by (at least) 20% (release, as total percentage)
        (self.simd() - other.simd()).into()
        // Self {
        //     x: self.x - other.x,
        //     y: self.y - other.y,
        //     z: self.z - other.z,
        // }
    }
}

impl From<Vec3> for Simd<f32, 3> {
    fn from(vec: Vec3) -> Self {
        Self::from_array([vec.x, vec.y, vec.z])
    }
}

impl From<Simd<f32, 3>> for Vec3 {
    fn from(simd: Simd<f32, 3>) -> Self {
        (simd[0], simd[1], simd[2]).into()
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
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        (self.simd() * other.simd()).reduce_sum()
        // self.x * other.x + self.y * other.y + self.z * other.z
    }
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        // Same perf as non-simd
        let s1 = self.simd();
        (s1.rotate_elements_left::<1>() * other.simd().rotate_elements_right::<1>()
            - s1.rotate_elements_right::<1>() * other.simd().rotate_elements_left::<1>())
        .into()
        // Self {
        //     x: self.y * other.z - self.z * other.y,
        //     y: self.z * other.x - self.x * other.z,
        //     z: self.x * other.y - self.y * other.x,
        // }
    }
    pub fn simd(self) -> Simd<f32, 3> {
        self.into()
    }
    pub fn len(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn len_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn normalize(&self) -> Self {
        let i_len = 1. / self.len();
        Self {
            x: self.x * i_len,
            y: self.y * i_len,
            z: self.z * i_len,
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
        *self + (2.0 * quat.w * uv) + (2.0 * uuv)
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
