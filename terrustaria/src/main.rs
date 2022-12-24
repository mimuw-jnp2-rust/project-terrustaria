use bevy::{math::Vec3Swizzles, prelude::*, time::FixedTimestep};
use bevy_ecs_tilemap::prelude::*;

mod map;
use map::{spawn_background, spawn_map, spawn_wall_map};

mod constants;
use constants::{BOUNDS, TIME_STEP, Z_FOREGROUND};

mod debug_helpers;
use debug_helpers::camera_debug_movement;

// Can be called with (x,y) transforming to (x,y,Z_FRGRND) or empty transforming to (0,0,Z_FRGRND)
macro_rules! bring_to_foreground {
    ($x:expr, $y:expr) => {
        Transform::from_xyz($x, $y, Z_FOREGROUND)
    };
    () => {
        Transform::from_xyz(0., 0., Z_FOREGROUND)
    };
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1270.0,
                        height: 720.0,
                        title: String::from("Terrustaria"),
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(TilemapPlugin)
        .add_startup_system(spawn_background)
        .add_startup_system(spawn_wall_map)
        .add_startup_system(spawn_map)
        .add_startup_system(setup)
        .add_system(camera_debug_movement)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system)
                .with_system(snap_to_player_system)
                .with_system(rotate_to_player_system),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player {
    // linear speed in meters per second
    movement_speed: f32,
    // rotation speed in radians per second
    rotation_speed: f32,
}

// snap to player ship behavior
#[derive(Component)]
struct SnapToPlayer;

// rotate to face player ship behavior
#[derive(Component)]
struct RotateToPlayer {
    // rotation speed in radians per second
    rotation_speed: f32,
}

// `X` axis goes from left to right (`+X` points right)
// `Y` axis goes from bottom to top (`+Y` point up)
// `Z` axis goes from far to near (`+Z` points towards you, out of the screen)
// The origin is at the center of the screen.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let ship_handle = asset_server.load("textures/simplespace/ship_C.png");
    let enemy_a_handle = asset_server.load("textures/simplespace/enemy_A.png");
    let enemy_b_handle = asset_server.load("textures/simplespace/enemy_B.png");

    let horizontal_margin = BOUNDS.x / 4.0;

    // player controlled ship
    commands.spawn((
        SpriteBundle {
            texture: ship_handle,
            transform: bring_to_foreground!(0., 50.),
            ..default()
        },
        Player {
            movement_speed: 500.0,                  // metres per second
            rotation_speed: f32::to_radians(360.0), // degrees per second
        },
    ));

    // enemy that snaps to face the player spawns on the left
    commands.spawn((
        SpriteBundle {
            texture: enemy_a_handle.clone(),
            transform: bring_to_foreground!(-horizontal_margin, 0.),
            ..default()
        },
        SnapToPlayer,
    ));

    // enemy that rotates to face the player enemy spawns on the right
    commands.spawn((
        SpriteBundle {
            texture: enemy_b_handle.clone(),
            transform: bring_to_foreground!(horizontal_margin, 0.),
            ..default()
        },
        RotateToPlayer {
            rotation_speed: f32::to_radians(45.0), // degrees per second
        },
    ));
}

// Demonstrates applying rotation and movement based on keyboard input.
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (ship, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        movement_factor -= 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship.rotation_speed * TIME_STEP);

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * ship.movement_speed * TIME_STEP;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, Z_FOREGROUND));
    transform.translation = transform.translation.min(extents).max(-extents);
}

// Demonstrates snapping the enemy ship to face the player ship immediately.
fn snap_to_player_system(
    mut query: Query<&mut Transform, (With<SnapToPlayer>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    // get the player translation in 2D
    let player_translation = player_transform.translation.xy();

    for mut enemy_transform in &mut query {
        // get the vector from the enemy ship to the player ship in 2D and normalize it.
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        // get the quaternion to rotate from the initial enemy facing direction to the direction
        // facing the player
        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));

        // rotate the enemy to face the player
        enemy_transform.rotation = rotate_to_player;
    }
}

// Demonstrates rotating an enemy ship to face the player ship at a given rotation speed.
fn rotate_to_player_system(
    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    // get the player translation in 2D
    let player_translation = player_transform.translation.xy();

    for (config, mut enemy_transform) in &mut query {
        // get the enemy ship forward vector in 2D (already unit length)
        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();

        // get the vector from the enemy ship to the player ship in 2D and normalize it.
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        // get the dot product between the enemy forward vector and the direction to the player.
        let forward_dot_player = enemy_forward.dot(to_player);

        // if the dot product is approximately 1.0 then the enemy is already facing the player and
        // we can early out.
        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        // get the right vector of the enemy ship in 2D (already unit length)
        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

        // find the angle
        let right_dot_player = enemy_right.dot(to_player);
        let rotation_sign = -f32::copysign(1.0, right_dot_player);
        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos();
        let rotation_angle = rotation_sign * (config.rotation_speed * TIME_STEP).min(max_angle);

        // rotate the enemy to face the player
        enemy_transform.rotate_z(rotation_angle);
    }
}
