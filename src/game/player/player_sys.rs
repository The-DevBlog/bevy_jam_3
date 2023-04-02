use bevy::prelude::*;

use super::player_cmps::*;
use crate::game::camera::camera_cmps::CustomCamera;
use crate::game::gamepad::gamepad_rcs::MyGamepad;

pub fn spawn_player(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = cmds
        .spawn((
            PbrBundle {
                material: materials.add(Color::BLUE.into()),
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: 0.5,
                    ..default()
                })),
                transform: Transform::from_xyz(0.0, 0.25, 0.0),
                ..default()
            },
            Player,
            Speed(5.0),
            Stamina(50.0),
            Health(100.0),
            IsSprinting(false),
            Name::new("Player"),
        ))
        .id();

    let translation = Vec3::new(0.0, 1.0, 2.0);
    let radius = translation.length();
    let camera = cmds
        .spawn((
            Camera3dBundle {
                transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            CustomCamera {
                radius,
                ..default()
            },
            Name::new("Camera"),
        ))
        .id();

    // make camera have same transform as player
    cmds.entity(player).push_children(&[camera]);
}

pub fn keyboard_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Speed, &mut IsSprinting), With<Player>>,
    cam_q: Query<&Transform, (With<CustomCamera>, Without<Player>)>,
) {
    for (mut transform, speed, mut sprinting) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::W) {
            direction += cam.forward().normalize();
        }

        // back
        if keys.pressed(KeyCode::S) {
            direction += cam.back().normalize();
        }

        // left
        if keys.pressed(KeyCode::A) {
            direction += cam.left().normalize();
        }

        // right
        if keys.pressed(KeyCode::D) {
            direction += cam.right().normalize();
        }

        // sprint
        let mut sprint = 1.0;
        if keys.pressed(KeyCode::LShift) {
            sprint = 1.4;
            sprinting.0 = true;
        }

        direction.y = 0.0;
        transform.translation += speed.0 * sprint * direction * time.delta_seconds();
    }
}

pub fn gamepad_movement(
    time: Res<Time>,
    axis: Res<Axis<GamepadAxis>>,
    btns: Res<Input<GamepadButton>>,
    mut player_q: Query<(&mut Transform, &Speed, &mut IsSprinting), With<Player>>,
    cam_q: Query<&Transform, (With<CustomCamera>, Without<Player>)>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    // return id of gamepad if one is connected
    let gamepad = if let Some(gp) = my_gamepad {
        gp.gamepad
    } else {
        return;
    };

    // get X & Y axis of left joystick
    let x_axis = GamepadAxis {
        axis_type: GamepadAxisType::LeftStickX,
        gamepad,
    };
    let y_axis = GamepadAxis {
        axis_type: GamepadAxisType::LeftStickY,
        gamepad,
    };

    let mut left_joystick = Vec2::ZERO;
    if let (Some(x), Some(y)) = (axis.get(x_axis), axis.get(y_axis)) {
        left_joystick = Vec2::new(x, y);
    }

    for (mut transform, speed, mut sprinting) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        if left_joystick.length() > 0.5 {
            // Get the direction of the joystick relative to the camera
            let forward = cam.forward().normalize();
            let right = cam.right().normalize();
            let mut joystick_direction = forward * left_joystick.y + right * left_joystick.x;
            joystick_direction.y = 0.0;
            joystick_direction = joystick_direction.normalize();

            // Move the player in the joystick direction
            direction += joystick_direction;
        }

        // sprint
        let mut sprint = 1.0;
        let left_thumb = GamepadButton::new(gamepad, GamepadButtonType::LeftThumb);
        if btns.pressed(left_thumb) {
            sprint = 1.4;
            sprinting.0 = true;
        }

        direction.y = 0.0;
        transform.translation += speed.0 * sprint * direction * time.delta_seconds();
    }
}

pub fn update_stamina(mut player_q: Query<(&mut Stamina, &mut IsSprinting), With<Player>>) {
    for (mut stamina, mut sprinting) in player_q.iter_mut() {
        println!("{}", sprinting.0);
        if sprinting.0 {
            stamina.0 -= 0.05;
        }

        sprinting.0 = false;
    }
}
