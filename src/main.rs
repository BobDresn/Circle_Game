use bevy::{
    app::AppExit,
    prelude::*,
    window::{PresentMode, PrimaryWindow}
};

pub mod components;
pub mod resources;
pub mod systems;
pub mod utilities;

use crate::components::*;
use crate::resources::*;
use crate::systems::*;
use crate::utilities::*;

const ENTITY_SIZE: f32 = 10.;
const ENTITY_SPEED: f32 = 500.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game".into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_event::<AppExit>()
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(3., TimerMode::Repeating)))

        .add_systems(Startup, (setup_window, setup, setup_enemy_pool, enemy_spawn).chain())
        .add_systems(PreUpdate, movement)
        .add_systems(PreUpdate, draw_circle)
        .add_systems(Update, enemy_spawn_timer)
        .add_systems(PostUpdate, check_collisions)
        .run();
}