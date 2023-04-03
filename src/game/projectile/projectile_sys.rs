use bevy::prelude::*;

use crate::game::{
    camera::camera_cmps::CustomCamera,
    enemy::enemy_cmps::Enemy,
    game_cmps::{Damage, Hp},
    gamepad::gamepad_rcs::MyGamepad,
    player::player_cmps::Player,
    world::MAP_SIZE,
};

use super::{projectile_cmps::Projectile, PROJECTILE_SPEED};

pub fn shoot_gamepad(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_q: Query<&Transform, With<Player>>,
    cam_q: Query<&Transform, (With<CustomCamera>, Without<Player>)>,
    btns: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    // return id of gamepad if one is connected
    let gamepad = if let Some(gp) = my_gamepad {
        gp.gamepad
    } else {
        return;
    };

    if let Ok(transform) = player_q.get_single() {
        let right_trigger = GamepadButton::new(gamepad, GamepadButtonType::RightTrigger2);

        if btns.just_pressed(right_trigger) {
            // Get the camera's forward direction vector on the xz plane
            let cam_transform = cam_q.iter().next().unwrap();

            cmds.spawn((
                PbrBundle {
                    material: materials.add(Color::YELLOW.into()),
                    mesh: meshes.add(Mesh::from(shape::UVSphere {
                        radius: 0.025,
                        ..default()
                    })),
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                },
                Projectile {
                    direction: Vec3::new(
                        cam_transform.translation.x,
                        0.0,
                        cam_transform.translation.z,
                    ),
                },
            ));
        }
    }
}

pub fn move_projectile(
    mut projectile_q: Query<(&mut Transform, &Projectile), With<Projectile>>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in projectile_q.iter_mut() {
        transform.translation -=
            projectile.direction.normalize() * PROJECTILE_SPEED * time.delta_seconds();
    }
}

pub fn damage_enemy(
    mut cmds: Commands,
    player_q: Query<&Damage, (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<(Entity, &Transform, &mut Hp), With<Enemy>>,
    projectile_q: Query<(Entity, &Transform, &Projectile), With<Projectile>>,
) {
    for (enemy_ent, enemy_transform, mut enemy_hp) in enemy_q.iter_mut() {
        for (projectile_ent, projectile_transform, projectile_dmg) in projectile_q.iter() {
            let distance = enemy_transform
                .translation
                .distance(projectile_transform.translation);

            let dmg = player_q.get_single().unwrap();

            // reduce enemy hp and despawn projectile
            if distance < 0.25 {
                // enemy_hp.0 -= projectile_dmg.damage;
                enemy_hp.0 -= dmg.0;
                cmds.entity(projectile_ent).despawn_recursive();
            }

            // despawn enemy
            if enemy_hp.0 <= 0.0 {
                cmds.entity(enemy_ent).despawn_recursive();
            }
        }
    }
}

/// despawn projectiles once they pass beyond the map bounds
pub fn despawn_projectile(
    mut cmds: Commands,
    projectile_q: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (ent, transform) in projectile_q.iter() {
        if transform.translation.x.abs() > MAP_SIZE / 2.0
            || transform.translation.z.abs() > MAP_SIZE / 2.0
        {
            cmds.entity(ent).despawn_recursive();
        }
    }
}
