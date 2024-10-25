use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Alive;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}