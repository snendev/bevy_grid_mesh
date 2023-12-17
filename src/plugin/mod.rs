use bevy::prelude::*;

use crate::{Chunk, Grid, GridChunkMesh, GridChunks, GridTracker, Vertex};

mod systems;

pub trait HeightMapFunc: Fn(f32, f32) -> f32 + Clone + Send + Sync + 'static {}

impl<F: Fn(f32, f32) -> f32 + Clone + Send + Sync + 'static> HeightMapFunc for F {}

#[derive(Clone, Resource)]
pub struct HeightMap<F: HeightMapFunc>(F);

impl<F: HeightMapFunc> HeightMap<F> {
    pub fn get_fn(&self) -> &F {
        &self.0
    }
}

impl<F: HeightMapFunc> From<F> for HeightMap<F> {
    fn from(xz_to_y: F) -> Self {
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

pub struct HeightMapPlugin<F: HeightMapFunc> {
    height_map: HeightMap<F>,
    material: StandardMaterial,
    tile_size: Vec2,
}

impl<F: HeightMapFunc> HeightMapPlugin<F> {
    pub fn new(
        height_map: impl Into<HeightMap<F>>,
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

impl<F: HeightMapFunc> Plugin for HeightMapPlugin<F> {
    fn build(&self, app: &mut App) {
        let mut materials = app.world.resource_mut::<Assets<StandardMaterial>>();
        let material_handle = materials.add(self.material.clone());

        app.insert_resource(ChunkMaterial::new(material_handle, self.tile_size))
            .insert_resource(self.height_map.clone())
            .add_systems(
                Update,
                (
                    systems::update_grid,
                    systems::attach_terrain,
                    systems::update_terrain_mesh::<F>,
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
