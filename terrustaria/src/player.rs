use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants;
use constants::{BOUNDS, TIME_STEP, Z_FOREGROUND};

use crate::helpers::bring_to_foreground;

#[derive(Component)]
pub struct Player {
    // linear speed in meters per second
    movement_speed: f32,
    // rotation speed in radians per second
    rotation_speed: f32,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle: Handle<Image> = asset_server.load("textures/simplespace/ship_C.png");

    commands
        .spawn((
            SpriteBundle {
                texture: ship_handle,
                // transform: bring_to_foreground!(0., 50.),
                ..default()
            },
            Player {
                movement_speed: 500.0,                  // metres per second
                rotation_speed: f32::to_radians(180.0), // degrees per second
            },
        ))
        .insert(Name::new("Player"))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(10.))
        .insert(GravityScale(0.))
        .insert(TransformBundle::from(bring_to_foreground!(0., 50.)));
}

// Demonstrates applying rotation and movement based on keyboard input.
pub fn player_movement(
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
