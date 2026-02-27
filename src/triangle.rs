use crate::vec3::Vec3;

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

impl From<Triangle> for FatTriangle {
    fn from(triangle: Triangle) -> Self {
        let v0v1 = triangle.v1 - triangle.v0;
        let v0v2 = triangle.v2 - triangle.v0;
        let normal = v0v1.cross(&v0v2).normalize();
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
