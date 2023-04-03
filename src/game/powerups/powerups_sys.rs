use bevy::prelude::*;
use rand::Rng;

use crate::game::{
    player::{
        player_cmps::{Player, Stamina},
        PLAYER_SIZE,
    },
    world::MAP_SIZE,
};

use super::{powerups_cmps::StaminaPowerUp, powerups_res::PowerUpSpawnTime};

pub fn spawn_stamina_powerups(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_timer: ResMut<PowerUpSpawnTime>,
    time: Res<Time>,
) {
    spawn_timer.0.tick(time.delta());

    let mut rng = rand::thread_rng();

    let map_bounds = MAP_SIZE / 2.0;
    let x = rng.gen_range(-map_bounds..=map_bounds);
    let z = rng.gen_range(-map_bounds..=map_bounds);

    if spawn_timer.0.finished() {
        cmds.spawn((
            PbrBundle {
                material: materials.add(Color::GREEN.into()),
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    height: 0.2,
                    radius: 0.1,
                    ..default()
                })),
                transform: Transform::from_xyz(x, 0.3, z),
                ..default()
            },
            StaminaPowerUp,
            Name::new("Stamina PowerUp"),
        ));
    }
}

pub fn collect_stamina_powerup(
    mut cmds: Commands,
    mut player_q: Query<(&mut Stamina, &Transform), With<Player>>,
    powerup_q: Query<(Entity, &Transform), With<StaminaPowerUp>>,
) {
    for (powerup_ent, powerup_transform) in powerup_q.iter() {
        for (mut stamina, player_transform) in player_q.iter_mut() {
            let distance = powerup_transform
                .translation
                .distance(player_transform.translation);

            // collect powerup and despawn
            if distance < PLAYER_SIZE {
                stamina.current = stamina.max;
                cmds.entity(powerup_ent).despawn_recursive();
            }
        }
    }
}
