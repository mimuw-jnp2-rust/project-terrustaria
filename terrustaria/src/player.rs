use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::{
    collision_groups::PLAYER_COLLIDE_WITH_ALL, depth::*, player::*, world::GRAVITY,
};

#[derive(Component)]
pub struct Player {
    // linear speed in meters per second
    movement_speed: f32,
}
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Jumper {
    jump_impulse: f32,
    is_jumping: bool,
}

pub fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut Velocity), With<Player>>,
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) && !jumper.is_jumping {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse);
            jumper.is_jumping = true
        }
    }
}

pub fn player_jump_reset(
    mut query: Query<(Entity, &mut Jumper)>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            set_jumping_false_if_touching_floor(entity, &mut jumper, contact_event);
        }
    }
}

fn set_jumping_false_if_touching_floor(
    entity: Entity,
    jumper: &mut Jumper,
    event: &CollisionEvent,
) {
    if let CollisionEvent::Started(h1, h2, ..) = event {
        if *h1 == entity || *h2 == entity {
            jumper.is_jumping = false
        }
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_handle: Handle<Image> = asset_server.load("player.png");
    commands
        .spawn((
            SpriteBundle {
                texture: player_handle,
                transform: Transform::from_xyz(0., 50., Z_PLAYER),
                ..default()
            },
            Player {
                movement_speed: MOVEMENT_SPEED, // metres per second
            },
        ))
        .insert(Jumper {
            jump_impulse: JUMP_POWER,
            is_jumping: false,
        })
        .insert(Name::new("Player"))
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(8., 8.))
        .insert(PLAYER_COLLIDE_WITH_ALL)
        .insert(GravityScale(GRAVITY))
        .insert(Velocity::zero())
        .with_children(|parent| {
            parent.spawn((Camera2dBundle::new_with_far(100.), MainCamera));
        });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            velocity.linvel = Vec2::new(-player.movement_speed, velocity.linvel.y);
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity.linvel = Vec2::new(player.movement_speed, velocity.linvel.y);
        }
    }
}
