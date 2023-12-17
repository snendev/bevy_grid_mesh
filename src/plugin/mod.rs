use bevy::prelude::*;

use crate::{Chunk, Grid, GridChunkMesh, GridChunks, GridTracker, Vertex};

mod systems;

// TODO: use dyn, Resource was having trouble before
pub type HeightMapFunc = fn(f32, f32) -> f32;

#[derive(Clone, Copy, Resource)]
pub struct HeightMap(HeightMapFunc);

impl HeightMap {
    pub fn get_fn(&self) -> &HeightMapFunc {
        &self.0
    }
}

impl From<HeightMapFunc> for HeightMap {
    fn from(xz_to_y: HeightMapFunc) -> Self {
        Self(xz_to_y)
    }
}

#[derive(Clone, Default, Resource)]
pub struct ChunkMaterial {
    pub material: Handle<StandardMaterial>,
    // the material will be tiled across the mesh using this tile_size (in transform space)
    pub tile_size: Vec2,
}

impl ChunkMaterial {
    pub fn new(material: Handle<StandardMaterial>, tile_size: Vec2) -> Self {
        Self {
            material,
            tile_size,
        }
    }
}

pub struct HeightMapPlugin {
    height_map: HeightMap,
    material: StandardMaterial,
    tile_size: Vec2,
}

impl HeightMapPlugin {
    pub fn new(
        height_map: impl Into<HeightMap>,
        material: impl Into<StandardMaterial>,
        tile_size: Vec2,
    ) -> Self {
        Self {
            height_map: height_map.into(),
            material: material.into(),
            tile_size,
        }
    }
}

impl Plugin for HeightMapPlugin {
    fn build(&self, app: &mut App) {
        let mut materials = app.world.resource_mut::<Assets<StandardMaterial>>();
        let material_handle = materials.add(self.material.clone());

        app.insert_resource(ChunkMaterial::new(material_handle, self.tile_size))
            .insert_resource(self.height_map)
            .add_systems(
                Update,
                (
                    systems::update_grid,
                    systems::attach_terrain,
                    systems::update_terrain_mesh,
                )
                    .chain(),
            );

        #[cfg(debug_assertions)]
        app.register_type::<Vertex>()
            .register_type::<Chunk>()
            .register_type::<Grid>()
            .register_type::<GridTracker>()
            .register_type::<GridChunks>()
            .register_type::<GridChunkMesh>();
    }
}
