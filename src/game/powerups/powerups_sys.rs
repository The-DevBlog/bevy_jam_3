use bevy::prelude::*;
use rand::Rng;

use crate::game::{
    game_cmps::Damage,
    player::{
        player_cmps::{Player, Stamina},
        PLAYER_SIZE,
    },
    world::MAP_SIZE,
};

use super::{
    powerups_cmps::{DamagePowerUp, HealthPowerUp, StaminaPowerUp},
    powerups_res::{DamageDuration, PowerUpSpawnTime},
    DMG_BOOST,
};

pub fn spawn_powerups(
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

    let mut powerup = |color: Color| -> PbrBundle {
        PbrBundle {
            material: materials.add(color.into()),
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                height: 0.2,
                radius: 0.1,
                ..default()
            })),
            transform: Transform::from_xyz(x, 0.3, z),
            ..default()
        }
    };

    if spawn_timer.0.finished() {
        let random_powerup = rng.gen_range(1..=3);

        match random_powerup {
            1 => {
                cmds.spawn((
                    powerup(Color::GREEN),
                    StaminaPowerUp,
                    Name::new("Stamina PowerUp"),
                ));
            }
            2 => {
                cmds.spawn((
                    powerup(Color::RED),
                    HealthPowerUp,
                    Name::new("Health PowerUp"),
                ));
            }
            3 => {
                cmds.spawn((
                    powerup(Color::YELLOW),
                    DamagePowerUp,
                    Name::new("Damage PowerUp"),
                ));
            }
            _ => (),
        }
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

pub fn collect_damage_powerup(
    mut cmds: Commands,
    mut player_q: Query<(&Transform, &mut Damage), With<Player>>,
    powerup_q: Query<(Entity, &Transform), With<DamagePowerUp>>,
    mut duration_res: ResMut<DamageDuration>,
) {
    for (powerup_ent, powerup_transform) in powerup_q.iter() {
        for (player_transform, mut dmg) in player_q.iter_mut() {
            let distance = powerup_transform
                .translation
                .distance(player_transform.translation);

            // collect powerup and despawn
            if distance < PLAYER_SIZE {
                duration_res.0.reset();
                duration_res.0.unpause();
                cmds.entity(powerup_ent).despawn_recursive();

                dmg.current = dmg.original + DMG_BOOST;
            }
        }
    }
}

pub fn tick_damage_duration_timer(
    mut player_q: Query<&mut Damage, With<Player>>,
    time: Res<Time>,
    mut duration_res: ResMut<DamageDuration>,
) {
    duration_res.0.tick(time.delta());

    if duration_res.0.finished() {
        if let Ok(mut dmg) = player_q.get_single_mut() {
            dmg.current = dmg.original;
        }

        duration_res.0.reset();
        duration_res.0.pause();
    }
}
