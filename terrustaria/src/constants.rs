use bevy::math::Vec2;
use bevy_ecs_tilemap::prelude::TilemapSize;

// Depth of field constants
pub const Z_BACKGROUND: f32 = 0.;
pub const Z_WALLS: f32 = 1.;
pub const Z_FOREGROUND: f32 = 5.;

// Movement constants
pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

// Map constants
const MAP_WIDTH: u32 = 64;
const MAP_DEPTH: u32 = 32;
pub const BUILDING_HEIGHT: u32 = 15;
pub const MAP_SIZE: TilemapSize = TilemapSize {
    x: MAP_WIDTH,
    y: MAP_DEPTH + BUILDING_HEIGHT,
};
