use bevy::prelude::*;

use crate::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Alive;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}

pub fn draw_enemies(mut gizmos: Gizmos, 
    enemy_query: Query<(&Transform, &Enemy), With<Alive>>
){
    for (transform, _enemy) in &enemy_query {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            ENTITY_SIZE,
            Color::srgb(255., 0., 255.),
        );
        
    }
}