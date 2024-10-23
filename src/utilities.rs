use bevy::math::Vec2;
use crate::ENTITY_SIZE;

pub fn check_circle_collision(a: Vec2, b: Vec2) -> bool {
    let distance_squared = (a - b).length_squared();
    let radius_squared = ENTITY_SIZE * 2.;
    distance_squared < radius_squared * radius_squared
}