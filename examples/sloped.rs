use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_grid_mesh::*;

fn sloped(_x: f32, z: f32) -> f32 {
    -z
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default(),
            HeightMapPlugin::new(sloped as fn(f32, f32) -> f32, Color::GRAY, Vec2::ONE * 100.),
        ))
        .add_systems(Startup, spawn)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(160., 160., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::X * 15. + Vec3::Y * 20.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        Name::new("Grid"),
        Grid::new(Vertex::new(100, 100), Vec2::ONE),
    ));
    commands.spawn((
        Name::new("Active Node"),
        GridTracker,
        SpatialBundle::default(),
    ));
}
