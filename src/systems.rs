use bevy::prelude::*;
use rand::{thread_rng, Rng};
use iyes_perf_ui::prelude::*;

use crate::*;

pub fn setup_window(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,) {
    let window = query.single();
    commands.insert_resource(WindowDimensions {
        width: window.width(),
        height: window.height(),
    });
}

pub fn setup(
    mut commands: Commands, 
    window: Res<WindowDimensions>,
) {
    let center = Vec2::new(window.width / 2., window.height / 2.);

    //Camera
    commands.spawn((
        Camera2dBundle{
            transform: Transform::from_xyz(center.x, center.y, 999.9),
            ..default()
        },
    ));

    //Player
    commands.spawn((
            Player,
            Transform::from_translation(Vec3::new(center.x, center.y, 0.)),
    ));

    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));
}

pub fn setup_enemy_pool(
    mut commands: Commands,
    window: Res<WindowDimensions>,
) {
    let mut rng = thread_rng();

    for _ in 0..100 {
        commands.spawn((
            Enemy,
            Velocity { 
                value: Vec3::new(
                    rng.gen_range(0. .. window.width), 
                    rng.gen_range(0. .. window.height), 
                    0.,
                )
            },
            Transform::from_translation(Vec3::new(
                rng.gen_range(-1000. ..=1000.), 
                rng.gen_range(-1000. ..=1000.), 
                0.,
            )),
        ));
    }
}

pub fn enemy_spawn(
    mut commands: Commands,
    query: Query<(Entity, &Enemy), Without<Alive>>,
) {
    for (entity, _enemy) in &query {
        commands.entity(entity).insert(Alive);
        break;
    }
}

pub fn enemy_spawn_timer(
    commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    query: Query<(Entity, &Enemy), Without<Alive>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        enemy_spawn(commands, query);
    }
}

//Handle keystrokes
pub fn movement(
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

pub fn handle_space (
    commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    query: Query<(Entity, &Alive)>,
    dead_enemies: Query<(Entity, &Enemy),Without<Alive>>,
    timer: ResMut<EnemySpawnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        match state.get() {
            GameState::Paused => next_state.set(GameState::Running),
            GameState::Start => next_state.set(GameState::Running),
            GameState::Running => next_state.set(GameState::Paused),
            GameState::GameOver => reset(commands, next_state, query, dead_enemies, timer),
        }
    }
}

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

pub fn draw_enemies(mut gizmos: Gizmos, 
    enemy_query: Query<&Transform, With<Alive>>
){
    for transform in &enemy_query {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            ENTITY_SIZE,
            Color::srgb(255., 0., 255.),
        );
        
    }
}

pub fn check_collisions(
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(&Transform, &Enemy), With<Alive>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let player_transform = &player_query.single();
    let player_center = Vec2::new(player_transform.translation.x, player_transform.translation.y);

    for (transform, _enemy) in &enemy_query {
        if transform.translation.x <= player_center.x + (ENTITY_SIZE * 2.) && transform.translation.x >= player_center.x - (ENTITY_SIZE * 2.) {
            let enemy_center = Vec2::new(transform.translation.x, transform.translation.y);
            if check_circle_collision(player_center, enemy_center) {
                next_state.set(GameState::GameOver);
            }
        }
    }
}

pub fn reset(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(Entity, &Alive)>,
    dead_enemies: Query<(Entity, &Enemy),Without<Alive>>,
    mut timer: ResMut<EnemySpawnTimer>,
) {
    for(entity, _alive) in &query {
        commands.entity(entity).remove::<Alive>();
    }
    timer.0.reset();
    next_state.set(GameState::Running);
    enemy_spawn(commands, dead_enemies);
}