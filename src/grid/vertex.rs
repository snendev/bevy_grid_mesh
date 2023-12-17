use derive_more::{Add, AddAssign, From, Mul, MulAssign, Sub, SubAssign};

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Add, AddAssign, Mul, MulAssign, Sub, SubAssign, From)]
#[derive(Reflect)]
pub struct Vertex {
    pub x: i32,
    pub z: i32,
}

impl Vertex {
    pub const ZERO: Vertex = Vertex::new(0, 0);

    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    // gets the nearest vertex to a given translation
    pub fn from_translation(translation: Vec3, quad_size: Vec2) -> Self {
        let nearest_vertex = Vec2::new(translation.x, translation.z) / quad_size;
        Self {
            x: nearest_vertex.x.round() as i32,
            z: nearest_vertex.y.round() as i32,
        }
    }
}
