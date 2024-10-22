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
        .add_systems(PreUpdate, direction_input)
//        .add_systems(Update, update_position)
        .add_systems(Update, log_fps)
        .add_systems(PostUpdate, draw_circle)
        .run();
}

///Will either be you or enemy
#[derive(Component)]
enum Team {
    Player, 
    //Enemy,
}

struct FrameCounter {
    frame_count: u32,
    elapsed_time: f32,
}

fn log_fps(
    time: Res<Time>,
    mut frame_counter: ResMut<FrameCounter>,
) {
    frame_counter.frame_count += 1;
    frame_counter.elapsed_time += time.delta_seconds();

    if frame_counter.elapsed_time >= 1.0 {
        let fps = frame_counter.frame_count as f32 / frame_counter.elapsed_time;
        println!("FPS: {:.2}", fps);
        frame_counter.frame_count = 0;
        frame_counter.elapsed_time = 0;
    }
}

//Spawns Player and enemy. Only ran on start. Will change to include menu UI and game state 
fn setup(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let window = query.single();
    let center = Vec2::new(window.width() / 2., window.height() / 2.);

    commands.insert_resource(FrameCounter {
        frame_count: 0,
        elapsed_time: 0.,
    });

    //Camera
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(center.x, center.y, 999.9),
        ..default()
    });

    //Player
    commands.spawn((
            Team::Player,
            Transform::from_translation(Vec3::new(center.x, center.y, 0.)),
    ));
    // commands.spawn(
    // (
    //     Team::Enemy,
    //     Position{ value: center }, 
    //     Velocity{ value: Vec2::new(100., 100.) },
    // ));
}

//Handle keystrokes
fn direction_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Team>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let (width, height) = (window.width(), window.height());

    for mut transform in &mut query {
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

        transform.translation.x = transform.translation.x.clamp(0., width);
        transform.translation.y = transform.translation.y.clamp(0., height);
    }
}

//Changes entity position each frame
// fn update_position(
//     window_query: Query<&Window, With<PrimaryWindow>>, 
//     mut query: Query<(&mut Position, &Velocity, &Team), With<Team>>,
//     time: Res<Time>,
//     ) {
//     let window = window_query.single();
//     let (width, height) = (window.width(), window.height());
//     let delta_seconds = time.delta_seconds();

//     for (mut position, velocity, team) in &mut query {
//         let displacement = velocity.value * delta_seconds;
//         position.value += displacement;

//         if position.value.x < 0.0 {
//             position.value.x = 0.0;
//         } else if position.value.x > width {
//             position.value.x = width;
//         }

//         if position.value.y < 0.0 {
//             position.value.y = 0.0;
//         } else if position.value.y > height {
//             position.value.y = height;
//         }
//     }
// }

fn draw_circle(
    mut gizmos: Gizmos, 
    query: Query<&Transform, With<Team>>
) {
    for transform in &query {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            ENTITY_SIZE,
            Color::WHITE,
        );
    }
}