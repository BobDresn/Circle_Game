use bevy::prelude::*;

use crate::*;

#[derive(Component)]
pub struct Player;

pub fn draw_player(
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<Player>>,
){
    for transform in &player_query {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            ENTITY_SIZE,
            Color::WHITE,
        );
    }
}