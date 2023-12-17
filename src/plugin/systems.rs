use bevy::prelude::*;

use crate::{
    Chunk, ChunkMaterial, Grid, GridChunkMesh, GridChunks, GridTracker, HeightMap, HeightMapFunc,
};

pub(super) fn update_grid(
    mut grid_query: Query<&mut Grid>,
    node_query: Query<&Transform, With<GridTracker>>,
) {
    let Ok(mut grid) = grid_query.get_single_mut() else {
        return;
    };
    let Ok(node) = node_query.get_single() else {
        return;
    };

    grid.update(node.translation);
}

pub(super) fn attach_terrain(mut commands: Commands, query: Query<Entity, Added<Grid>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(GridChunks::default());
    }
}

pub(super) fn update_terrain_mesh<F: HeightMapFunc>(
    mut commands: Commands,
    mut chunk_query: Query<(&Grid, &mut GridChunks)>,
    height_map: Res<HeightMap<F>>,
    chunk_material: Option<Res<ChunkMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (grid, mut chunks) in chunk_query.iter_mut() {
        // remove out-of-bounds chunks
        let chunks_to_remove = chunks
            .chunk_entities
            .iter()
            .filter_map(|(vertex, _)| {
                if !grid.chunks_in_play.contains(vertex) {
                    Some(*vertex)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for vertex in chunks_to_remove {
            if let Some(entities) = chunks.chunk_entities.remove(&vertex) {
                for entity in entities {
                    commands.entity(entity).despawn();
                }
            }
        }

        // spawn missing in-bounds chunks
        for origin in grid.chunks_in_play.iter() {
            if !chunks.chunk_entities.contains_key(origin) {
                let chunk = Chunk {
                    quad_size: grid.quad_size,
                    size: grid.chunk_size,
                    origin: *origin,
                };
                let grid_chunk_bundle = GridChunkMesh::new(chunk, Vec2::new(8., 8.)).to_bundle(
                    &height_map,
                    chunk_material.as_deref(),
                    &mut meshes,
                );
                let chunk_entity = commands.spawn(grid_chunk_bundle).id();
                chunks.chunk_entities.insert(*origin, vec![chunk_entity]);
            }
        }
    }
}
