use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;

use crate::constants::*;

//todo: Losowwanie mapy - algorytm losujący z różnym prawdopodobieństwem różne rudy, można rozszerzyć na jaskinie
//todo: Przyciemnić miejsca, których nie widzimy pod ziemią
//todo: Interakcja Myszka-Kafelek
//todo: Niszczenie kafelków
//todo: Zmiana Struktur na ładniejsze, w szczególności bez ramek
//todo: Chunking?
//todo: przesuwanie tła w momencie kiedy gracz się porusza/zapętlenie tła/zmiana na basic kolor

//move top of the map to the middle of world space, takes into account building height
fn transform_map(
    size: &TilemapSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    z: f32,
) -> Transform {
    let low = TilePos::new(0, 0).center_in_world(grid_size, map_type);
    let high =
        TilePos::new(size.x - 1, size.y - 1 - BUILDING_HEIGHT).center_in_world(grid_size, map_type);

    let diff = high - low;

    Transform::from_xyz(-diff.x / 2., -diff.y, z)
}

struct TileType {
    name: String,
    occurrence_prob: f32,
    texture_index: TileTextureIndex,
    valid: Box<dyn Fn(u32, u32) -> bool>,
}

impl TileType {
    fn new(
        name: String,
        occurrence_prob: f32,
        texture_index: TileTextureIndex,
        valid: impl Fn(u32, u32) -> bool + 'static,
    ) -> Self {
        Self {
            name,
            occurrence_prob,
            texture_index,
            valid: Box::new(valid),
        }
    }
}

fn init_tile_types() -> Vec<TileType> {
    vec![
        TileType::new(String::from("Grass"), 0.6, TileTextureIndex(0), |_, _| true),
        TileType::new(String::from("Stone"), 0.3, TileTextureIndex(3), |_, y| {
            y < 5
        }),
        TileType::new(String::from("Water"), 0.1, TileTextureIndex(1), |x, y| {
            y < 10 && x % 2 == 0
        }),
    ]
}

fn get_random_tile_type(tile_types: &[TileType]) -> usize {
    let mut val = thread_rng().gen();
    for (i, tile_type) in tile_types.iter().enumerate() {
        let prob = tile_type.occurrence_prob;
        if prob >= val {
            return i;
        } else {
            val -= prob;
        }
    }
    0
}

fn fill_tilemap_without_building_area(
    texture_index: TileTextureIndex,
    map_size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    map_name: &str,
) {
    let tile_types = init_tile_types();

    for x in 0..map_size.x {
        for y in 0..map_size.y - BUILDING_HEIGHT {
            let tile_pos = TilePos { x, y };
            let mut idx: usize;
            loop {
                idx = get_random_tile_type(&tile_types);
                if (tile_types[idx].valid)(x, y) {
                    break;
                }
            }

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id,
                    texture_index: tile_types[idx].texture_index,
                    ..Default::default()
                })
                .insert(Name::new(format!("{map_name}Tile({x},{y})")))
                // .insert(RigidBody::Fixed)
                // .insert(Collider::cuboid(16., 16.))
                // .insert(TransformBundle::from(Transform::from_xyz(
                //     (x * 16) as f32,
                //     (y * 16) as f32,
                //     0.,
                // )))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}

fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_id: u32,
    z_translation: f32,
    map_name: &str,
) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands
        .spawn_empty()
        .insert(Name::new(format!("{map_name}Map")))
        .id();

    fill_tilemap_without_building_area(
        TileTextureIndex(texture_id),
        MAP_SIZE,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
        map_name,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: transform_map(&MAP_SIZE, &grid_size, &map_type, z_translation),
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
    spawn_map(commands, asset_server, 3, Z_WALLS, "Wall");
}

pub fn spawn_foreground_map(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_map(commands, asset_server, 0, Z_FOREGROUND, "Foreground");
}
