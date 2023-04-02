use bevy::prelude::*;
use rand::random;

use crate::game::{
    game_cmps::{Health, Speed},
    world::MAP_SIZE,
};

use super::{enemy_cmps::Enemy, ENEMY_HEALTH, ENEMY_SPEED};

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
        Enemy,
        Speed(ENEMY_SPEED),
        Health(ENEMY_HEALTH),
        Name::new("Enemy"),
    ));
}
