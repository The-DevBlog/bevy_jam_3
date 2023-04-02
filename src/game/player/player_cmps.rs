use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Stamina(pub f32);

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct IsSprinting(pub bool);
