use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::{Chunk, ChunkMaterial, HeightMap, HeightMapFunc, Vertex};

#[derive(Debug, Clone, Default)]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GridChunkMesh {
    // the chunk being rendered
    pub chunk: Chunk,
    pub texture_tile_size: Vec2,
}

impl GridChunkMesh {
    pub fn new(chunk: Chunk, texture_tile_size: Vec2) -> Self {
        Self {
            chunk,
            texture_tile_size,
        }
    }

    // get the triangles to render the quad with origin at local_vertex
    fn get_quad_triangles(&self, local_vertex: Vertex) -> [u32; 6] {
        let row_offset = self.chunk.size.x as u32 + 1;
        let quad_index = row_offset * local_vertex.z as u32 + local_vertex.x as u32;
        [
            // right triangle
            quad_index + row_offset + 1,
            quad_index + 1,
            quad_index + row_offset,
            // left triangle
            quad_index,
            quad_index + row_offset,
            quad_index + 1,
        ]
    }

    pub fn count_vertices(&self) -> usize {
        (self.chunk.count_columns() * self.chunk.count_rows()) as usize
    }

    pub fn count_indices(&self) -> usize {
        // Each grid square is defined by a quad, which is 6 vertices
        self.chunk.area() as usize * 6
    }

    fn build_mesh(&self, height_map: impl Fn(f32, f32) -> f32) -> (Mesh, Vec3) {
        let num_vertices = self.count_vertices();
        let num_indices = self.count_indices();
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

        let origin = self.chunk.origin_vertex();
        let origin_y = height_map(origin.x as f32, origin.z as f32);

        for vertex in self.chunk.iter_by_row() {
            // the translation in local space
            let vertex_position = self.chunk.translation(vertex);
            // the vertex coordinate in global space
            let global_vertex = origin + vertex;

            let height_sample = height_map(global_vertex.x as f32, global_vertex.z as f32);
            let position = Vec3::new(
                vertex_position.x,
                height_sample - origin_y,
                vertex_position.y,
            );

            positions.push(position.to_array());
            normals.push(Vec3::Y.to_array());
            uvs.push([
                global_vertex.z as f32 / self.texture_tile_size.x,
                global_vertex.x as f32 / self.texture_tile_size.y,
            ]);

            if vertex.x < self.chunk.size.x && vertex.z < self.chunk.size.z {
                indices.extend_from_slice(&self.get_quad_triangles(vertex));
            }
        }

        let mesh = Mesh::new(PrimitiveTopology::TriangleList)
            .with_indices(Some(Indices::U32(indices)))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        let Vec2 {
            x: translation_x,
            y: translation_z,
        } = self.chunk.translation(origin);
        (mesh, Vec3::new(translation_x, origin_y, translation_z))
    }

    pub fn to_bundle<F: HeightMapFunc>(
        self,
        height_map: &HeightMap<F>,
        chunk_material: Option<&ChunkMaterial>,
        meshes: &mut Assets<Mesh>,
    ) -> impl Bundle {
        let (mesh, origin) = self.build_mesh(height_map.get_fn());
        (
            Name::new(format!(
                "Terrain Chunk {}x{}",
                self.chunk.origin.x, self.chunk.origin.z,
            )),
            PbrBundle {
                mesh: meshes.add(mesh),
                material: chunk_material.cloned().unwrap_or_default().material,
                transform: Transform::from_translation(origin),
                ..Default::default()
            },
            self,
        )
    }
}
