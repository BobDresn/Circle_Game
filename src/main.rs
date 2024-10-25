use bevy::{
    app::AppExit,
    prelude::*,
    window::{PresentMode, PrimaryWindow}
};
use iyes_perf_ui::prelude::*;

pub mod components;
pub mod gamestate;
pub mod resources;
pub mod systems;
pub mod utilities;

use crate::components::*;
use crate::gamestate::*;
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
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(3., TimerMode::Repeating)))
        .insert_state(GameState::Start)
        .add_event::<AppExit>()
        .add_systems(Startup, (setup_window, setup, setup_enemy_pool, enemy_spawn).chain())
        .add_systems(PreUpdate, handle_space)
        .add_systems(PreUpdate, movement.run_if(in_state(GameState::Running)))
        .add_systems(PreUpdate, (draw_player, draw_enemies).chain().run_if(in_state(GameState::Running)))
        .add_systems(Update, enemy_spawn_timer.run_if(in_state(GameState::Running)))
        .add_systems(PostUpdate, check_collisions.run_if(in_state(GameState::Running)))
        .run();
}