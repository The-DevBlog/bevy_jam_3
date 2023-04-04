use bevy::prelude::*;

use crate::game::game_cmps::Game;

use super::MAP_SIZE;

pub fn spawn_ground(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: MAP_SIZE,
                ..default()
            })),
            material: materials.add(Color::GRAY.into()),
            ..default()
        },
        Game,
        Name::new("Ground"),
    ));
}

pub fn spawn_light(mut cmds: Commands) {
    cmds.spawn((
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Game,
        Name::new("Point Light"),
    ));
}
