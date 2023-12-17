use itertools::Itertools;

use bevy::prelude::*;

use super::Vertex;

// Handles chunking in 2D vertices using a consistent grid of `Vertex` in quad_size units
#[derive(Debug, Clone)]
#[derive(Reflect)]
pub struct Chunk {
    // the length of the chunk in vertices
    pub size: Vertex,
    // the chunk's origin in chunk coordinates
    pub origin: Vertex,
    // the quad size of each chunk in the transform space
    pub quad_size: Vec2,
}

impl Chunk {
    pub fn new(origin: Vertex, size: Vertex, quad_size: Vec2) -> Self {
        Self {
            size,
            origin,
            quad_size,
        }
    }

    // gets the nearest chunk to a given translation
    pub fn from_translation(translation: Vec3, chunk_size: Vertex, quad_size: Vec2) -> Self {
        let origin = Vertex::from_translation(
            translation,
            // scale quads by chunk_size to get position in chunk units rather than grid units
            quad_size * Vec2::new(chunk_size.x as f32, chunk_size.z as f32),
        );
        Self::new(origin, chunk_size, quad_size)
    }

    pub fn count_columns(&self) -> i32 {
        self.size.x + 1
    }

    pub fn count_rows(&self) -> i32 {
        self.size.z + 1
    }

    pub fn area(&self) -> i32 {
        self.size.x * self.size.z
    }

    pub fn width(&self) -> i32 {
        self.size.x
    }

    pub fn depth(&self) -> i32 {
        self.size.z
    }

    pub fn origin_vertex(&self) -> Vertex {
        Vertex::new(self.origin.x * self.size.x, self.origin.z * self.size.z)
    }

    pub fn raw_width(&self) -> f32 {
        self.width() as f32 * self.quad_size.x
    }

    pub fn raw_depth(&self) -> f32 {
        self.depth() as f32 * self.quad_size.y
    }

    pub fn translation(&self, vertex: Vertex) -> Vec2 {
        Vec2::new(
            vertex.x as f32 * self.quad_size.x,
            vertex.z as f32 * self.quad_size.y,
        )
    }

    pub fn iter_by_row(&self) -> impl Iterator<Item = Vertex> {
        (0..=self.size.z)
            .cartesian_product(0..=self.size.x)
            .map(|(z, x)| Vertex { x, z })
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new((0, 0).into(), (40, 40).into(), Vec2::ONE * 2.)
    }
}
