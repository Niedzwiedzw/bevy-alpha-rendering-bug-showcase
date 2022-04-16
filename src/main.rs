use bevy::{
    prelude::*,
    render::render_resource::Face,
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_translation((3., 0., 3.).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            alpha_mode: AlphaMode::Blend,
            cull_mode: Some(Face::Back),
            ..default()
        }),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_translation((3.2, 0., 3.2).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            alpha_mode: AlphaMode::Blend,
            cull_mode: Some(Face::Back),
            ..default()
        }),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_flycam::PlayerPlugin)
        .add_startup_system(setup)
        .run();
}
