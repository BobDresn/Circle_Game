use bevy::prelude::*;

#[derive(Resource)]
pub struct WindowDimensions {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);
