use std::thread::sleep;
use bevy::{prelude::*, math::Vec4Swizzles};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use crate::cursor::CursorPos;
use crate::map::WithColliders;
use crate::player::Player;


pub fn destroy_tile_after_click(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    mut tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &mut TileStorage,
        &Transform,
    ), With<WithColliders>>,
    mut tile_q: Query<&mut TileTextureIndex>,
    player_q: Query<(&Player, &Velocity)>,
    mouse: Res<Input<MouseButton>>,
) {

    for (map_size, grid_size, map_type, mut tile_storage, map_transform) in tilemap_q.iter_mut() {
        let cursor_pos: Vec3 = cursor_pos.0;
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        // We have only one player
        let (_, player_velocity) = player_q.single();
        let player_moving_fast = player_velocity.linvel.length() > 0.2;

        // Skip when mouse is not pressed and destroy only when player is not moving fast
        if !mouse.pressed(MouseButton::Left) || player_moving_fast {
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
                    }
                    else {
                        tile_texture.0 += 1;
                        sleep(std::time::Duration::from_millis(100));
                    }
                }
            }
        }
    }
}
