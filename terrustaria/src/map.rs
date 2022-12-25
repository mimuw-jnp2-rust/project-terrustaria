use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

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

fn fill_tilemap_without_building_area(
    texture_index: TileTextureIndex,
    map_size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    map_name: &str,
) {
    for x in 0..map_size.x {
        for y in 0..map_size.y - BUILDING_HEIGHT {
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

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>, texture_id: u32,  z_translation: f32, map_name: &str) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity =
        commands.spawn_empty()
            .insert(Name::new(format!("{map_name}Map")))
            .id();

    fill_tilemap_without_building_area(
        TileTextureIndex(texture_id),
        MAP_SIZE,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
        map_name
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

    commands.spawn(SpriteBundle {
        texture: background,
        transform: Transform::from_xyz(0.0, 0.0, Z_BACKGROUND),
        ..Default::default()
    })
    .insert(Name::new("Background"));
}

pub fn spawn_wall_map(commands: Commands,
                             asset_server: Res<AssetServer>) {
    spawn_map(commands, asset_server, 3, Z_WALLS, "Wall");
}

pub fn spawn_foreground_map(commands: Commands,
                        asset_server: Res<AssetServer>) {
    spawn_map(commands, asset_server, 0, Z_FOREGROUND, "Foreground");

}

