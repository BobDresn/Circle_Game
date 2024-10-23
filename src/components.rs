use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub alive: bool
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}