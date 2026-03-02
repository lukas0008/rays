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
        .filter_map(|line| {
            if line.starts_with("f ") {
                let mut line = line.split(" ");
                line.next()?;
                Some(Triangle::new(
                    verticies[get_v(&mut line)?],
                    verticies[get_v(&mut line)?],
                    verticies[get_v(&mut line)?],
                    0xFFFFFF.into(),
                ))
            } else {
                None
            }
        })
        .map(Into::into)
        .collect();

    Some(Model::new(triangles))
}
