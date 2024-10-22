use bevy::{
    app::AppExit,
    prelude::*,
    window::{PresentMode, PrimaryWindow}
};
use rand::Rng;

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
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, movement)
        .add_systems(Update, draw_circle)
        .add_systems(PostUpdate, check_collisions)
        //.add_systems(PostUpdate, print_position)
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

    let mut rng = rand::thread_rng();
    let rand_x = rng.gen_range(0. .. window.width());
    let rand_y = rng.gen_range(0. .. window.height());
    let rand_vel_x = rng.gen_range(-1000. ..=1000.);
    let rand_vel_y = rng.gen_range(-1000. ..=1000.);

    commands.spawn((
        Enemy,
        Velocity{ value: Vec3::new(rand_vel_x, rand_vel_y, 0.) },
        Transform::from_translation(Vec3::new(rand_x, rand_y, 0.)),
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
        let enemy_center = Vec2::new(transform.translation.x, transform.translation.y);
        if check_circle_collision(player_center, enemy_center) {
            exit_events.send(AppExit::Success);
        }
    }
}

// fn print_position( 
//     query: Query<&Transform, With<Enemy>> 
// ) {
//     for transform in &query {
//         println!("{}, {}", transform.translation.x, transform.translation.y);
//     }
// }