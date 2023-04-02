use bevy::prelude::*;

use crate::game::{
    camera::camera_cmps::CustomCamera,
    gamepad::gamepad_rcs::MyGamepad,
    player::player_cmps::{Player, Projectile},
};

use super::PROJECTILE_SPEED;

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

pub fn projectile_movement(
    mut projectile_q: Query<(&mut Transform, &Projectile), With<Projectile>>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in projectile_q.iter_mut() {
        transform.translation -=
            projectile.direction.normalize() * PROJECTILE_SPEED * time.delta_seconds();
    }
}
