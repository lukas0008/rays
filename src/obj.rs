use std::ops::Add;

use crate::{model::Model, quat::Quat, triangle::Triangle, vec3::Vec3};

pub fn parse_obj(obj: &str) -> Option<Model> {
    let obj = obj
        .lines()
        .filter(|line| !line.starts_with("#"))
        .collect::<Vec<_>>();
    fn get_f<'a>(i: &mut impl Iterator<Item = &'a str>) -> Option<f32> {
        i.next().map(str::parse).map(Result::ok)?
    }
    fn get_v<'a>(i: &mut impl Iterator<Item = &'a str>) -> Option<usize> {
        i.next()?
            .split("/")
            .next()?
            .parse()
            .ok()
            .map(|v: usize| v - 1)
    }
    let verticies = obj
        .iter()
        .filter_map(|line| {
            if line.starts_with("v ") {
                let mut cords = line.split(" ");
                cords.next()?;
                Some((get_f(&mut cords)?, get_f(&mut cords)?, get_f(&mut cords)?).into())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let triangles = obj
        .iter()
        .flat_map(|line| {
            if line.starts_with("f ") {
                let mut line = line.split(" ");
                line.next()?;
                let v1 = verticies[get_v(&mut line)?];
                let v2 = verticies[get_v(&mut line)?];
                let v3 = verticies[get_v(&mut line)?];
                let triangle = Triangle::new(v1, v2, v3, 0xFFFFFF.into());
                if let Some(v4) = get_v(&mut line) {
                    let v4 = verticies[v4];
                    Some([
                        Some(triangle),
                        Some(Triangle::new(v2, v4, v3, 0xFFFFFF.into())),
                    ])
                } else {
                    Some([Some(triangle), None])
                }
            } else {
                None
            }
        })
        .flatten()
        .flatten()
        .map(Into::into)
        .collect();

    Some(Model::new(triangles))
}
