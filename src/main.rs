use bevy::{
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
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, movement)
        .add_systems(Update, draw_circle)
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

//Spawns Player and enemy. Only ran on start. Will change to include menu UI and game state 
fn setup(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let window = query.single();
    let center = Vec2::new(window.width() / 2., window.height() / 2.);

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

    commands.spawn((
        Enemy,
        Velocity{ value: Vec3::new(5., 1000., 0.) },
        Transform::from_translation(Vec3::new(0., 0., 0.)),
    ));
}

//Handle keystrokes
fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, Option<&Player>, Option<&mut Velocity>), With<Transform>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let (width, height) = (window.width(), window.height());

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
    
            if transform.translation.x > width || transform.translation.x < 0. {
                velocity.value.x *= -1.;
            }
            if transform.translation.y > height || transform.translation.y < 0. {
                velocity.value.y *= -1.;
            }
        }
        transform.translation.x = transform.translation.x.clamp(0., width);
        transform.translation.y = transform.translation.y.clamp(0., height);
    }
}

fn draw_circle(
    mut gizmos: Gizmos, 
    query: Query<(&Transform, Option<&Player>, Option<&Enemy>)>
) {
    //Draw for player
    for (transform, player, enemy) in &query {
        if player.is_some() {
            gizmos.circle_2d(
                Vec2::new(transform.translation.x, transform.translation.y),
                ENTITY_SIZE,
                Color::WHITE,
            );
        }
        if enemy.is_some() {
            if player.is_some() {
                gizmos.circle_2d(
                    Vec2::new(transform.translation.x, transform.translation.y),
                    ENTITY_SIZE,
                    Color::srgb(255., 0., 255.),
                );
            }
        }
    }
}