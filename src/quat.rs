use std::ops::Mul;

#[derive(Clone, Copy, Debug)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<(f32, f32, f32, f32)> for Quat {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self { x, y, z, w }
    }
}

impl Quat {
    pub fn from_euler_angles(pitch: f32, yaw: f32, roll: f32) -> Self {
        let (sp, cp) = pitch.to_radians().sin_cos();
        let (sy, cy) = yaw.to_radians().sin_cos();
        let (sr, cr) = roll.to_radians().sin_cos();

        Self {
            w: cp * cy * cr + sp * sy * sr,
            x: sp * cy * cr - cp * sy * sr,
            y: cp * sy * cr + sp * cy * sr,
            z: cp * cy * sr - sp * sy * cr,
        }
    }
    pub fn identity() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 1.,
        }
    }
}

impl Mul<Quat> for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
    }
}
