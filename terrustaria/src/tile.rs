use bevy_ecs_tilemap::prelude::*;

use crate::constants::{map::MAP_SIZE, offsets::*};

pub struct TileType {
    #[allow(dead_code)]
    name: String,
    rarity: f32,
    texture_index: TileTextureIndex,
    valid: Box<dyn Fn(u32, u32) -> bool>,
}

impl TileType {
    fn new(
        name: String,
        rarity: f32,
        tile_offset: u32,
        valid: impl Fn(u32, u32) -> bool + 'static,
    ) -> Self {
        Self {
            name,
            rarity,
            texture_index: TileTextureIndex(tile_offset * 5),
            valid: Box::new(valid),
        }
    }

    pub fn get_rarity(&self) -> f32 {
        self.rarity
    }

    pub fn get_texture_index(&self) -> TileTextureIndex {
        self.texture_index
    }

    pub fn is_valid(&self, pos: &TilePos) -> bool {
        (self.valid)(pos.x, pos.y) && pos.x < MAP_SIZE.x && pos.y < MAP_SIZE.y
    }
}

pub struct TileCollection {
    types: Vec<TileType>,
}

impl TileCollection {
    pub fn new() -> Self {
        Self {
            types: vec![
                TileType::new(String::from("Dirt"), 60.0, DIRT_OFFSET, |_, _| true),
                TileType::new(String::from("Stone"), 10.0, STONE_OFFSET, |_, y| y < 12),
                TileType::new(String::from("Water"), 10.0, WATER_OFFSET, |_, y| y < 20),
                TileType::new(String::from("Diamond"), 1., DIAMOND_OFFSET, |_, y| y < 12),
            ],
        }
    }

    pub fn get_tiles(&self) -> &Vec<TileType> {
        &self.types
    }

    pub fn at(&self, idx: usize) -> &TileType {
        &self.types[idx]
    }

    pub fn stone_tile(&self) -> &TileType {
        self.at(1)
    }

    pub fn rarity_sum_valid(&self, pos: &TilePos) -> f32 {
        let mut sum: f32 = 0.0;
        for tile_type in &self.types {
            if tile_type.is_valid(pos) {
                sum += tile_type.get_rarity();
            }
        }
        sum
    }
}
