use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::constants;
use constants::{world::{BOUNDS, TIME_STEP}, depth::Z_FOREGROUND};

use crate::helpers::bring_to_foreground;

use crate::player::Player;

#[derive(Component)]
pub struct SnapToPlayer;

#[derive(Component)]
pub struct RotateToPlayer {
    // rotation speed in radians per second
    rotation_speed: f32,
}

pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_a_handle = asset_server.load("textures/simplespace/enemy_A.png");
    let enemy_b_handle = asset_server.load("textures/simplespace/enemy_B.png");

    let horizontal_margin = BOUNDS.x / 4.0;

    // enemy that snaps to face the player spawns on the left
    commands
        .spawn((
            SpriteBundle {
                texture: enemy_a_handle,
                ..default()
            },
            SnapToPlayer,
        ))
        .insert(Name::new("Enemy left"))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(10., 10.))
        .insert(TransformBundle::from(bring_to_foreground!(
            -horizontal_margin,
            100.
        )));

    // enemy that rotates to face the player enemy spawns on the right
    commands
        .spawn((
            SpriteBundle {
                texture: enemy_b_handle,
                // transform: bring_to_foreground!(horizontal_margin, 0.),
                ..default()
            },
            RotateToPlayer {
                rotation_speed: f32::to_radians(45.0),
            },
        ))
        .insert(Name::new("Enemy right"))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(10., 10.))
        .insert(TransformBundle::from(bring_to_foreground!(
            horizontal_margin,
            100.
        )));
}

// snapping the enemy ship to face the player ship immediately
pub fn snap_to_player(
    mut query: Query<&mut Transform, (With<SnapToPlayer>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    // get the player translation in 2D
    let player_translation = player_transform.translation.xy();

    for mut enemy_transform in &mut query {
        // get the vector from the enemy ship to the player ship in 2D and normalize it.
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));
        // rotate the enemy to face the player
        enemy_transform.rotation = rotate_to_player;
    }
}

// rotating an enemy ship to face the player ship at a given rotation speed
pub fn rotate_to_player(
    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    // get the player translation in 2D
    let player_translation = player_transform.translation.xy();

    for (config, mut enemy_transform) in &mut query {
        // get the enemy ship forward vector in 2D (already unit length)
        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
        let forward_dot_player = enemy_forward.dot(to_player);
        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

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
