use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::constants::*;
use crate::tile::*;

#[derive(Component)]
pub struct WithColliders;


fn random_in_range(range: f32) -> f32 {
    let val: f32 = thread_rng().gen();
    val * range
}

fn random_u32(a: u32, b: u32) -> u32 {
    let val: f32 = thread_rng().gen();
    (((b - a) as f32) * val - 0.001) as u32 + a
}

fn get_random_tile_type(tile_types: &TileCollection, pos: &TilePos) -> usize {
    let rarity_sum = tile_types.rarity_sum_valid(pos);
    let mut random = random_in_range(rarity_sum);
    for (i, tile_type) in tile_types.get_tiles().iter().enumerate() {
        if tile_type.is_valid(pos) {
            if random < tile_type.get_rarity() {
                return i;
            } else {
                random -= tile_type.get_rarity();
            }
        }
    }
    0
}

fn create_cave(
    tile_types: &TileCollection,
    visited: &mut Vec<Vec<bool>>,
    start_pos: TilePos,
    mut size: u32,
) -> Vec<TilePos> {
    if MAX_CAVE_SIZE < size {
        size = MAX_CAVE_SIZE;
    }

    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, -1, 0, 1];
    let mut in_cave = vec![start_pos];
    visited[start_pos.x as usize][start_pos.y as usize] = true;
    let mut processed: usize = 0;
    let start_size = size as f32;
    let stone_tile = tile_types.stone_tile();

    while processed < in_cave.len() && size > 0 {
        let pos = in_cave[processed];
        for i in 0..4 {
            let new_pos = TilePos {
                x: (pos.x as i32 + dx[i]) as u32,
                y: (pos.y as i32 + dy[i]) as u32,
            };
            if stone_tile.is_valid(&new_pos) && !visited[new_pos.x as usize][new_pos.y as usize] {
                // Some randomization.
                if random_in_range(start_size) <= start_size - (processed as f32) {
                    visited[new_pos.x as usize][new_pos.y as usize] = true;
                    in_cave.push(new_pos);
                }
            }
        }
        processed += 1;
        size -= 1;
    }
    in_cave
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
fn fill_tilemap_randomly(
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    map_name: &str,
) {
    let tile_types = TileCollection::new();
    let mut visited = vec![vec![false; MAP_SIZE.y as usize]; MAP_SIZE.x as usize];

    // Create a cave.
    let start_x = random_u32(0, MAP_SIZE.x);
    let start_y = random_u32(0, 10);
    let in_cave = create_cave(
        &tile_types,
        &mut visited,
        TilePos {
            x: start_x,
            y: start_y,
        },
        MAX_CAVE_SIZE,
    );
    for tile_pos in in_cave {
        let x = tile_pos.x;
        let y = tile_pos.y;
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id,
                texture_index: tile_types.stone_tile().get_texture_index(),
                ..Default::default()
            })
            .insert(Name::new(format!("{map_name}Tile({x},{y})")))
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    }

    // Fill standard tiles.
    for x in 0..MAP_SIZE.x {
        for y in 0..MAP_SIZE.y - BUILDING_HEIGHT {
            if visited[x as usize][y as usize] {
                continue;
            } else {
                visited[x as usize][y as usize] = true;
            }

            let tile_pos = TilePos { x, y };
            let mut idx: usize;
            loop {
                idx = get_random_tile_type(&tile_types, &tile_pos);
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
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}

pub fn spawn_colliders(
    mut commands: Commands,
    tilemap_q: Query<&TileStorage, With<WithColliders>>,
    tile_q: Query<&mut TilePos>,
) {
    for tilemap_storage in tilemap_q.iter() {
        for tile_entity in tilemap_storage.iter().flatten() {
            let tile_pos = tile_q.get(*tile_entity).unwrap();
            let transform_bundle = TransformBundle::from(Transform::from_translation(
                (Vec2::new((tile_pos.x * 16) as f32 - 8., (tile_pos.y * 16) as f32 - 8.) + map_transform_vec2())
                    .extend(0.)));

            commands.entity(*tile_entity)
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(16., 16.))
                .insert(transform_bundle);
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
        commands.entity(tilemap_entity).insert(WithColliders);
        fill_tilemap_randomly(
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
