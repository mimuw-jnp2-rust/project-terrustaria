use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::constants::*;
use crate::tile::*;


fn get_random_tile_type(tile_types: &TileCollection) -> usize {
    let mut val = thread_rng().gen();
    for (i, tile_type) in tile_types.get_tiles().iter().enumerate() {
        let prob = tile_type.get_prob();
        if prob >= val {
            return i;
        } else {
            val -= prob;
        }
    }
    0
}

// Fills the tilemap with set texture_id, does not fill building area.
fn fill_tilemap_with_set_structure_id(
    texture_index: TileTextureIndex,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    map_name: &str,
) {
    for x in 0..MAP_SIZE.x {
        for y in 0..MAP_SIZE.y - BUILDING_HEIGHT {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id,
                    texture_index,
                    ..Default::default()
                })
                .insert(Name::new(format!("{map_name}Tile({x},{y})")))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}

// Fills randomly tilemap with colliders and textures, does not fill building area.
fn fill_tilemap_randomly_with_colliders(
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    map_name: &str,
) {
    let tile_types = TileCollection::new();

    for x in 0..MAP_SIZE.x {
        for y in 0..MAP_SIZE.y - BUILDING_HEIGHT {
            let tile_pos = TilePos { x, y };
            let mut idx: usize;
            loop {
                idx = get_random_tile_type(&tile_types);
                if tile_types.at(idx).is_valid(&tile_pos) {
                    break;
                }
            }

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id,
                    texture_index: tile_types.at(idx).get_texture_index(),
                    ..Default::default()
                })
                .insert(Name::new(format!("{map_name}Tile({x},{y})")))
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(16., 16.))
                .insert(TransformBundle::from(Transform::from_translation(
                    (Vec2::new((x * 16) as f32 - 8., (y * 16) as f32 - 8.) + map_transform_vec2())
                        .extend(0.),
                )))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}

fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    z_translation: f32,
    map_name: &str,
) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands
        .spawn_empty()
        .insert(Name::new(format!("{map_name}Map")))
        .id();

    if map_name == "Wall" {
        fill_tilemap_with_set_structure_id(
            TileTextureIndex(3),
            TilemapId(tilemap_entity),
            &mut commands,
            &mut tile_storage,
            map_name,
        );
    } else if map_name == "Foreground" {
        fill_tilemap_randomly_with_colliders(
            TilemapId(tilemap_entity),
            &mut commands,
            &mut tile_storage,
            map_name,
        );
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: GRID_SIZE,
        map_type: MAP_TYPE,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        transform: Transform::from_translation(map_transform_vec2().extend(z_translation)),
        ..Default::default()
    });
}

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background: Handle<Image> = asset_server.load("background.png");

    commands
        .spawn(SpriteBundle {
            texture: background,
            transform: Transform::from_xyz(0.0, 0.0, Z_BACKGROUND),
            ..Default::default()
        })
        .insert(Name::new("Background"));
}

pub fn spawn_wall_map(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_map(commands, asset_server, Z_WALLS, "Wall");
}

pub fn spawn_foreground_map(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_map(commands, asset_server, Z_FOREGROUND, "Foreground");
}
