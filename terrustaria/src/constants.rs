use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::{
    TilePos, TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType,
};

// depth of field constants
pub const Z_BACKGROUND: f32 = 0.;
pub const Z_WALLS: f32 = 1.;
pub const Z_FOREGROUND: f32 = 5.;

// movement constants
pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

// color constants
pub const TRANSPARENT: Color = Color::Rgba {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 0.0,
};

// map constants
const MAP_WIDTH: u32 = 64;
const MAP_DEPTH: u32 = 32;
pub const BUILDING_HEIGHT: u32 = 15;
pub const MAP_SIZE: TilemapSize = TilemapSize {
    x: MAP_WIDTH,
    y: MAP_DEPTH + BUILDING_HEIGHT,
};

pub const MAP_TYPE: TilemapType = TilemapType::Square;
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16., y: 16. };
pub const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 16., y: 16. };

// counts x and y translation of map, that top middle tile of the map is located in (0.0)
pub fn map_transform_vec2() -> Vec2 {
    let low = TilePos::new(0, 0).center_in_world(&GRID_SIZE, &MAP_TYPE);
    let high = TilePos::new(MAP_SIZE.x - 1, MAP_SIZE.y - 1 - BUILDING_HEIGHT)
        .center_in_world(&GRID_SIZE, &MAP_TYPE);

    let diff = high - low;

    Vec2::new(-diff.x / 2., -diff.y)
}
