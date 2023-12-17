# bevy_grid_mesh

Grid-Mesh is a rudimentary prototype for a simple procedural mesh builder for Bevy. It provides infinite 3D procedural mesh generation using a height map function `(x: f32, z: f32) -> f32`.

`bevy_grid_mesh` was built to mimic the API from the Godot plugin [hterrain](https://github.com/Zylann/godot_heightmap_plugin), which slices a plane into a grid and builds a mesh based on the heights defined for each coordinate on the grid. The current implementation is probably less involved than hterrain, but it does the basic job of creating a mesh based on any defined heightmap.

## Usage

Add the plugin to your Bevy app.

```rs
use bevy::prelude::*;
use bevy_grid_mesh::*;

fn my_height_map(x: f32, z: f32) -> f32 {
    // ...
}

fn main() {
    App::new()
        // ...
        .add_plugins(
            HeightMapPlugin::new(
                // the height_map which determines the appropriate y for each vertex
                my_height_map,
                // any impl Into<StandardMaterial>, the texture used to tile mesh chunks
                Color::GRAY,
                // the tile_size for the chunk texture
                Vec2::ONE * 100.,
            ),
        )
        .run();
}

// ...
```

Then, spawn a `Grid` and a `GridTracker` which will act as the "centerpoint" for the rendered chunks.

```rs
fn spawn(mut commands: Commands) {
    commands.spawn(
        Grid::new(
            // chunk_size, the size in "vertex units" of each chunk
            Vertex::new(100, 100),
            // quad_size, the size of each mesh quad in transform units
            Vec2::ONE,
        )
    );
    commands.spawn((
        // this tells the Level where to "center" the map
        // as the translation moves, new nearby chunks are spawned and faraway ones are despawned
        GridTracker,
        SpatialBundle::default(),
    ));
}
```

Tip: If you are using `bevy_xpbd` or `bevy_rapier`, you can use
[`bevy_xpbd_3d::AsyncCollider`](https://github.com/Jondolf/bevy_xpbd/blob/58d3f97b207615bb629e3675d30cc0a1aaeeee62/src/components/collider.rs#L861)
or
[`bevy_rapier_3d::Collider::from_bevy_mesh`](https://github.com/dimforge/bevy_rapier/blob/c6bcce4695d596a7a9c8e91748d4dbb3d31f6d13/src/geometry/collider_impl.rs#L174)
to build a collider using the mesh.

```rs
// bevy_xpbd_3d
fn attach_chunk_colliders(
    mut commands: Commands,
    added_chunks_query: Query<Entity, Added<GridChunkMesh>>,
) {
    for entity in added_chunks_query.iter() {
        commands.entity(entity).insert(
            AsyncCollider(ComputedCollider::TriMesh),
            // ...
            // other physics components
        );
    }
}

// bevy_rapier3d
fn attach_chunk_colliders(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    added_chunks_query: Query<(Entity, &Handle<Mesh>), Added<GridChunkMesh>>,
) {
    for (entity, mesh_handle) in added_chunks_query.iter() {
        let mesh = meshes.get(mesh_handle).unwrap();
        let shape = ComputedColliderShape::TriMesh;
        commands.entity(entity).insert(
            Collider::from_bevy_mesh(mesh, &shape).unwrap(),
            // ...
            // other physics components
        );
    }
}
```

(Be sure to order the system so that the mesh and physics components are added within the same frame and before physics runs, to avoid
[1-frame delays](https://bevy-cheatbook.github.io/programming/system-order.html?highlight=delay#does-it-even-matter).)

## Examples

Try cloning and running an example!

```sh
cargo run --example $EXAMPLE
```

- `jagged` builds a plane that is jaggedly bent along the z-axis
- `sloped` builds a constant downward slope
- `noisy` builds a mesh whose vertices are all represented by a noise generator

## TODOS

- Render larger, lower-resolution chunks at far distances to render more space while staying cheap
- Render better normals
- Rename the crate to something better
- Trim dependencies

## History

`bevy_grid_mesh` is based on the terrain generation code for [Cheese Rolling Forever](https://github.com/snendev/cheese-rolling-forever), a game built for Bevy Jam 4.

## License

Dual-licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
