use crate::constants::map::{map_transform_vec2, TILE_SIZE};
use crate::cursor::CursorPos;
use crate::map::WithColliders;
use crate::player::Player;
use bevy::{math::Vec4Swizzles, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use std::thread::sleep;

pub fn destroy_tile_after_click(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    mut tilemap_q: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &mut TileStorage,
            &Transform,
        ),
        With<WithColliders>,
    >,
    mut tile_q: Query<&mut TileTextureIndex>,
    player_q: Query<(&Player, &Velocity)>,
    position_q: Query<&GlobalTransform, With<Player>>,
    mouse: Res<Input<MouseButton>>,
) {
    let player_pos = position_q.single().translation();

    for (map_size, grid_size, map_type, mut tile_storage, map_transform) in tilemap_q.iter_mut() {
        let cursor_pos: Vec3 = cursor_pos.0;
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        // we have only one player
        let (_, player_velocity) = player_q.single();
        let player_moving_fast = player_velocity.linvel.length() > 0.2;

        // skip when mouse is not pressed and destroy only when player is not moving fast
        if !mouse.pressed(MouseButton::Left) || player_moving_fast {
            continue;
        }

        // check if player is in range of a tile
        let eps_x: f32 = 1.5 * TILE_SIZE.x;
        let eps_y: f32 = 1.5 * TILE_SIZE.y;
        let dif_x: f32 = cursor_in_map_pos.x - player_pos.x + map_transform_vec2().x;
        let dif_y: f32 = cursor_in_map_pos.y - player_pos.y + map_transform_vec2().y;
        if dif_x.abs() > eps_x || dif_y.abs() > eps_y {
            continue;
        }

        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                if let Ok(mut tile_texture) = tile_q.get_mut(tile_entity) {
                    if tile_texture.0 % 5 == 4 {
                        commands.entity(tile_entity).despawn_recursive();
                        tile_storage.remove(&tile_pos);
                    } else {
                        tile_texture.0 += 1;
                        sleep(std::time::Duration::from_millis(100));
                    }
                }
            }
        }
    }
}
