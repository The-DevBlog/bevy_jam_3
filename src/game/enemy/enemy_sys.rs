use bevy::prelude::*;
use rand::random;

use crate::game::{
    game_cmps::{Hp, Speed},
    world::MAP_SIZE,
};

use super::{enemy_cmps::Enemy, ENEMY_HP, ENEMY_SPEED};

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
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.25,
                depth: 0.25,
                ..default()
            })),
            transform: Transform::from_xyz(x, 0.3, z),
            ..default()
        },
        Enemy,
        Speed(ENEMY_SPEED),
        Hp(ENEMY_HP),
        Name::new("Enemy"),
    ));
}
