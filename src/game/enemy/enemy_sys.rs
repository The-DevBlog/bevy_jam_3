use bevy::{ecs::query::QueryIter, prelude::*};
use rand::Rng;

use crate::game::{
    game_cmps::{Game, Hp, Speed},
    player::player_cmps::Player,
    world::MAP_SIZE,
};

use super::{
    enemy_cmps::{CircularCollider, Enemy},
    enemy_res::EnemySpawnTimer,
    ENEMY_HP, ENEMY_SPEED,
};

pub fn spawn_enemies(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
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
            CircularCollider(0.25),
            Speed(ENEMY_SPEED),
            Hp(ENEMY_HP),
            Game,
            Name::new("Enemy"),
        ));
    }
}

/// enemies track towards player
pub fn enemy_tracking(
    mut enemy_q: Query<(&mut Transform, &Speed), With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for (mut enemy_trans, enemy_speed) in enemy_q.iter_mut() {
        if let Ok(player_trans) = player.get_single() {
            let direction = (player_trans.translation - enemy_trans.translation).normalize();

            enemy_trans.translation += direction * enemy_speed.0 * time.delta_seconds();
        }
    }
}

pub fn enemy_collision(// mut enemy_q: Query<(&mut Transform, &CircularCollider), With<Enemy>>,
    // q_2: QueryIter<&CircularCollider, With<Enemy>>,
) {
    // let count = q_2.count();
    // let count = enemy_q.iter().count();
    // println!("ENEMIES: {}", count);
}

// pub fn enemy_collision(mut query: Query<&mut Transform, With<Enemy>>) {
//     let mut transforms: Vec<Transform> = query.iter_mut().map(|t| *t).collect();

//     for (i, mut transform1) in transforms.iter().enumerate() {
//         for mut transform2 in transforms.iter().skip(i + 1) {
//             let distance = transform1.translation.distance(transform2.translation);

//             if distance < 0.5 {
//                 let direction = (transform1.translation - transform2.translation).normalize();
//                 let delta_translation = direction * (1.0 - distance) / 2.0;

//                 // Adjust the translation of the first transform
//                 let mut new_translation = transform1.translation + delta_translation;
//                 new_translation.y = transform1.translation.y; // maintain the same height
//                 transform1.translation = new_translation;
//                 // query.get_mut(transforms[i].entity).unwrap().translation = new_translation;

//                 // Adjust the translation of the second transform
//                 let mut new_translation = transform2.translation - delta_translation;
//                 new_translation.y = transform2.translation.y; // maintain the same height
//                 transform2.translation = new_translation;
//                 // query.get_mut(transforms[i + 1].entity).unwrap().translation = new_translation;
//             }
//         }
//     }
// }
