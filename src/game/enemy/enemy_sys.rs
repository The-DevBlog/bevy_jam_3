use bevy::prelude::*;
use rand::random;

use crate::game::{
    player::player_cmps::{Health, Player, Speed},
    world::MAP_SIZE,
};

pub fn spawn_enemy(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let x = random::<f32>() * MAP_SIZE / 2.0;
    let z = random::<f32>() * MAP_SIZE / 2.0;

    cmds.spawn((
        PbrBundle {
            material: materials.add(Color::RED.into()),
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 0.5,
                ..default()
            })),
            transform: Transform::from_xyz(x, 0.25, z),
            ..default()
        },
        Player,
        Speed(5.0),
        Health(100.0),
        Name::new("Enemy"),
    ));
}
