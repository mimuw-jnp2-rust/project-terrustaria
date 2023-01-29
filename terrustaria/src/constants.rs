// depth of field constants
pub mod depth {
    pub const Z_BACKGROUND: f32 = 0.;
    pub const Z_WALLS: f32 = 0.01;
    pub const Z_FOREGROUND: f32 = 0.02;
    pub const Z_PLAYER: f32 = 0.03;
}

// tile offsets for tile_strip.png texture asset
pub mod offsets {
    pub const DIRT_OFFSET: u32 = 0;
    pub const STONE_OFFSET: u32 = 1;
    pub const WATER_OFFSET: u32 = 2;
    pub const DIAMOND_OFFSET: u32 = 3;
}

// Group collisions constants
pub mod collision_groups {
    use bevy_rapier2d::geometry::{CollisionGroups, Group};

    const MAP_GROUP: Group = Group::from_bits_truncate(0b0001); // membership group [0]
    const PLAYER_GROUP: Group = Group::from_bits_truncate(0b0010); // membership group [1]

    pub const MAP_COLLIDE_WITH_ALL_EXCEPT_MAP: CollisionGroups = CollisionGroups {
        memberships: MAP_GROUP,
        filters: Group::from_bits_truncate(u32::MAX << 1),
    };

    pub const PLAYER_COLLIDE_WITH_ALL: CollisionGroups = CollisionGroups {
        memberships: PLAYER_GROUP,
        filters: Group::ALL,
    };
}

pub mod world {
    use bevy::math::Vec2;

    pub const GRAVITY: f32 = 3.;
    pub const PHYSICS_SCALE: f32 = 1.;

    pub const TIME_STEP: f32 = 1.0 / 60.0;
    pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
}

pub mod player {
    pub const JUMP_POWER: f32 = 120.;
    pub const MOVEMENT_SPEED: f32 = 100.;
}

// map constants
pub mod map {
    use bevy::math::Vec2;
    use bevy_ecs_tilemap::prelude::{
        TilePos, TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType,
    };

    const MAP_WIDTH: u32 = 64;
    const MAP_DEPTH: u32 = 32;
    pub const BUILDING_HEIGHT: u32 = 15;
    pub const MAP_SIZE: TilemapSize = TilemapSize {
        x: MAP_WIDTH,
        y: MAP_DEPTH + BUILDING_HEIGHT,
    };

    pub const MAP_TYPE: TilemapType = TilemapType::Square;
    pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 20., y: 20. };
    pub const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 20., y: 20. };
    pub const COLLIDER_SIZE: Vec2 = Vec2::new(TILE_SIZE.x / 2., TILE_SIZE.y / 2.);

    pub const MAX_CAVE_SIZE: u32 = 150;

    // counts x and y translation of map, that top middle tile of the map is located in (0.0)
    pub fn map_transform_vec2() -> Vec2 {
        let low = TilePos::new(0, 0).center_in_world(&GRID_SIZE, &MAP_TYPE);
        let high = TilePos::new(MAP_SIZE.x - 1, MAP_SIZE.y - 1 - BUILDING_HEIGHT)
            .center_in_world(&GRID_SIZE, &MAP_TYPE);

        let diff = high - low;

        Vec2::new(-diff.x / 2., -diff.y)
    }
}
