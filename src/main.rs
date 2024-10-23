use std::sync::{Arc, Mutex};
use rand::prelude::*;
use bevy::{
    app::AppExit,
    prelude::*,
    window::{PresentMode, PrimaryWindow}
};

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
        .insert_resource(RandomNumberGenerator::new())
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(5., TimerMode::Repeating)))
        .add_systems(Startup, (setup_window, setup, spawn_initial_enemy).chain())
        .add_systems(PreUpdate, (movement, draw_circle).chain())
        .add_systems(Update, enemy_spawn_timer)
        .add_systems(PostUpdate, check_collisions)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Velocity {
    value: Vec3,
}

#[derive(Resource)]
struct WindowDimensions {
    width: f32,
    height: f32,
}

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource)]
struct RandomNumberGenerator {
    rng: Arc<Mutex<StdRng>>,
}

impl RandomNumberGenerator {
    fn new() -> Self {
        let rng = StdRng::from_entropy();
        RandomNumberGenerator {
            rng: Arc::new(Mutex::new(rng)),
        }
    }
}

fn setup_window(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,) {
    let window = query.single();
    commands.insert_resource(WindowDimensions {
        width: window.width(),
        height: window.height(),
    });
}

fn setup(
    mut commands: Commands, 
    window: Res<WindowDimensions>,
) {
    let center = Vec2::new(window.width / 2., window.height / 2.);

    //Camera
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(center.x, center.y, 999.9),
        ..default()
    });

    //Player
    commands.spawn((
            Player,
            Transform::from_translation(Vec3::new(center.x, center.y, 0.)),
    ));
}

fn enemy_spawn(
    mut commands: Commands,
    rng: Res<RandomNumberGenerator>,
    window: Res<WindowDimensions>,
) {
    //Unlocks rng generator
    //Removes overhead of creating new generator every time
    let mut rng_locked = rng.rng.lock().unwrap();

    //Gets coordinates inside window bounds
    let rand_x = rng_locked.gen_range(0. .. window.width);
    let rand_y = rng_locked.gen_range(0. .. window.height);

    //Gets random vector direction speeds
    let rand_vel_x = rng_locked.gen_range(-1000. ..=1000.);
    let rand_vel_y = rng_locked.gen_range(-1000. ..=1000.);

    commands.spawn((
        Enemy,
        Velocity{ value: Vec3::new(rand_vel_x, rand_vel_y, 0.) },
        Transform::from_translation(Vec3::new(rand_x, rand_y, 0.)),
    ));
}

fn spawn_initial_enemy(
    commands: Commands,
    rng: Res<RandomNumberGenerator>,
    window: Res<WindowDimensions>,
) {
    enemy_spawn(commands, rng, window);
}

fn enemy_spawn_timer(
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    commands: Commands,
    rng: Res<RandomNumberGenerator>,
    window: Res<WindowDimensions>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        enemy_spawn(commands, rng, window);
    }
}

//Handle keystrokes
fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, Option<&Player>, Option<&mut Velocity>), With<Transform>>,
    time: Res<Time>,
    window: Res<WindowDimensions>,
) {
    for (mut transform, player, velocity) in &mut query {
        if player.is_some() {
            let mut direction = Vec3::ZERO;

            if input.pressed(KeyCode::KeyW) {
                direction.y += 1.;
            }
            if input.pressed(KeyCode::KeyA) {
                direction.x -= 1.;
            }
            if input.pressed(KeyCode::KeyS) {
                direction.y -= 1.;
            }
            if input.pressed(KeyCode::KeyD) {
                direction.x += 1.;
            }

            if direction != Vec3::ZERO {
                direction = direction.normalize();
            }

            transform.translation += direction * ENTITY_SPEED * time.delta_seconds();
        }
        if let Some(mut velocity) = velocity {
            velocity.value = velocity.value.normalize();
            transform.translation += velocity.value * ENTITY_SPEED * time.delta_seconds();
    
            if transform.translation.x > window.width || transform.translation.x < 0. {
                velocity.value.x *= -1.;
            }
            if transform.translation.y > window.height || transform.translation.y < 0. {
                velocity.value.y *= -1.;
            }
        }
        transform.translation.x = transform.translation.x.clamp(0., window.width);
        transform.translation.y = transform.translation.y.clamp(0., window.height);
    }
}

fn draw_circle(
    mut gizmos: Gizmos, 
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    //Draw for player
    for transform in &player_query {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            ENTITY_SIZE,
            Color::WHITE,
        );
    }
    //Draw enemy
    for transform in &enemy_query {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            ENTITY_SIZE,
            Color::srgb(255., 0., 255.),
        );
    }
}

fn check_circle_collision(a: Vec2, b: Vec2) -> bool {
    let distance_squared = (a - b).length_squared();
    let radius_squared = ENTITY_SIZE * 2.;
    distance_squared < radius_squared * radius_squared
}

fn check_collisions(
    mut exit_events: EventWriter<AppExit>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    let player_transform = &player_query.single();
    let player_center = Vec2::new(player_transform.translation.x, player_transform.translation.y);

    for transform in &enemy_query {
        if transform.translation.x <= player_center.x + (ENTITY_SIZE * 2.) && transform.translation.x >= player_center.x - (ENTITY_SIZE * 2.) {
            let enemy_center = Vec2::new(transform.translation.x, transform.translation.y);
            if check_circle_collision(player_center, enemy_center) {
                exit_events.send(AppExit::Success);
            }
        }
        
    }
}