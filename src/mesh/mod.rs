use bevy::{prelude::*, utils::HashMap};

mod chunk;
pub use chunk::*;

use crate::Vertex;

#[derive(Clone, Debug, Default)]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GridChunks {
    pub chunk_entities: HashMap<Vertex, Vec<Entity>>,
}

impl GridChunks {
    pub fn new() -> Self {
        Self {
            chunk_entities: HashMap::new(),
        }
    }

    pub fn name() -> Name {
        Name::new("Terrain")
    }
}
