use bevy::math::{Vec2, Vec3};
use bevy_ecs_tilemap::prelude::{TilemapGridSize, TilemapSize};
use bevy_ecs_tilemap::tiles::TilePos;

// Depth of field constants
pub const Z_BACKGROUND: f32 = 0.;
pub const Z_WALLS: f32 = 1.;
pub const Z_FOREGROUND: f32 = 5.;

// Movement constants
pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

// Player constants
pub const PLAYER_POS: Vec2 = Vec2::new(100., 100.);


// Camera constants
pub const CAMERA_POS: Vec3 = PLAYER_POS.extend(1000.);

// Map constants
const MAP_WIDTH: u32 = 32;
const MAP_DEPTH: u32 = 16;
pub const BUILDING_HEIGHT: u32 = 15;
pub const MAP_SIZE: TilemapSize = TilemapSize {
    x: MAP_WIDTH,
    y: MAP_DEPTH + BUILDING_HEIGHT,
};

pub const TILE_SIZE: TilemapSize = TilemapSize {
    x: 16,
    y: 16,
};
pub const GRID_SIZE: TilemapGridSize = TilemapGridSize {x: 16., y:16.};
