/// This module defines types that help build out the grid space in which the chunks are built.
use itertools::Itertools;

use bevy::{prelude::*, utils::HashSet};

mod chunk;
pub use chunk::*;

mod vertex;
pub use vertex::*;

// The grid extends the XZ plane into chunks and tracks which entities are inside which chunk
#[derive(Clone, Debug)]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Grid {
    // in Chunk units
    pub chunks_in_play: HashSet<Vertex>,
    // in Grid units
    pub chunk_size: Vertex,
    pub quad_size: Vec2,
}

impl Grid {
    const VISIBLE_CHUNKS_RANGE: (i32, i32) = (3, 3);

    pub fn new(chunk_size: Vertex, quad_size: Vec2) -> Self {
        Self {
            chunks_in_play: HashSet::default(),
            chunk_size,
            quad_size,
        }
    }

    pub fn name() -> Name {
        Name::new("Grid")
    }

    pub fn update(&mut self, cheese_position: Vec3) {
        let cheese_chunk =
            Chunk::from_translation(cheese_position, self.chunk_size, self.quad_size);

        let left_edge = cheese_chunk
            .origin
            .x
            .saturating_sub(Self::VISIBLE_CHUNKS_RANGE.0);
        let right_edge = cheese_chunk
            .origin
            .x
            .saturating_add(Self::VISIBLE_CHUNKS_RANGE.0);
        let forward_edge = cheese_chunk
            .origin
            .z
            .saturating_add(Self::VISIBLE_CHUNKS_RANGE.1);
        let backward_edge = cheese_chunk
            .origin
            .z
            .saturating_sub(Self::VISIBLE_CHUNKS_RANGE.1);

        self.chunks_in_play.clear();
        for (x, y) in (left_edge..=right_edge).cartesian_product(backward_edge..=forward_edge) {
            let chunk_vertex = (x, y).into();
            self.chunks_in_play.insert(chunk_vertex);
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(Vertex::new(40, 40), Vec2::ONE * 2.)
    }
}

// TODO: if this is always placed on a camera (rather than e.g. a player character),
// we could track z-rotation (or maybe the frustum?) to despawn everything out of vision
// Perhaps we support a GridCharacterTracker and a GridCameraTracker
#[derive(Copy, Clone, Debug, Default)]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GridTracker;
