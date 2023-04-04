use bevy::prelude::*;

use crate::game::{
    camera::camera_cmps::CustomCamera,
    enemy::enemy_cmps::Enemy,
    game_cmps::{Damage, Game, Hp},
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
    mouse: Res<Input<MouseButton>>,
) {
    // return id of gamepad if one is connected
    let gamepad = if let Some(gp) = my_gamepad {
        gp.gamepad
    } else {
        return;
    };

    if let Ok(player_trans) = player_q.get_single() {
        let right_trigger = GamepadButton::new(gamepad, GamepadButtonType::RightTrigger2);

        if btns.just_pressed(right_trigger) || mouse.just_pressed(MouseButton::Left) {
            // Get the camera's forward direction vector on the xz plane
            let cam_trans = cam_q.iter().next().unwrap();

            cmds.spawn((
                PbrBundle {
                    material: materials.add(Color::YELLOW.into()),
                    mesh: meshes.add(Mesh::from(shape::UVSphere {
                        radius: 0.025,
                        ..default()
                    })),
                    transform: Transform::from_translation(player_trans.translation),
                    ..default()
                },
                Projectile {
                    direction: Vec3::new(cam_trans.translation.x, 0.0, cam_trans.translation.z),
                },
                Game,
            ));
        }
    }
}

pub fn move_projectile(
    mut projectile_q: Query<(&mut Transform, &Projectile), With<Projectile>>,
    time: Res<Time>,
) {
    for (mut trans, projectile) in projectile_q.iter_mut() {
        trans.translation -=
            projectile.direction.normalize() * PROJECTILE_SPEED * time.delta_seconds();
    }
}

pub fn dmg_enemy(
    mut cmds: Commands,
    player_q: Query<&Damage, (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<(Entity, &Transform, &mut Hp), With<Enemy>>,
    projectile_q: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (enemy_ent, enemy_trans, mut enemy_hp) in enemy_q.iter_mut() {
        for (projectile_ent, projectile_trans) in projectile_q.iter() {
            let distance = enemy_trans
                .translation
                .distance(projectile_trans.translation);

            let dmg = player_q.get_single().unwrap();

            // reduce enemy hp and despawn projectile
            if distance < 0.25 {
                // enemy_hp.0 -= projectile_dmg.damage;
                enemy_hp.0 -= dmg.current;
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
    for (ent, trans) in projectile_q.iter() {
        if trans.translation.x.abs() > MAP_SIZE / 2.0 || trans.translation.z.abs() > MAP_SIZE / 2.0
        {
            cmds.entity(ent).despawn_recursive();
        }
    }
}
