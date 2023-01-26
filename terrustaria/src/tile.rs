use bevy_ecs_tilemap::prelude::*;

pub const MAX_CAVE_SIZE: u32 = 50;

pub struct TileType {
    #[allow(dead_code)]
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

    pub fn get_prob(&self) -> f32 {
        self.occurrence_prob
    }

    pub fn get_texture_index(&self) -> TileTextureIndex {
        self.texture_index
    }

    pub fn is_valid(&self, pos: &TilePos) -> bool {
        (self.valid)(pos.x, pos.y)
    }
}

pub struct TileCollection {
    types: Vec<TileType>,
}

impl TileCollection {
    pub fn new() -> Self {
        Self {
            types: vec![
                TileType::new(String::from("Grass"), 0.6, TileTextureIndex(0), |_, _| true),
                TileType::new(String::from("Stone"), 0.3, TileTextureIndex(3), |_, y| {
                    y < 12
                }),
                TileType::new(String::from("Water"), 0.1, TileTextureIndex(1), |x, y| {
                    y < 20 && x % 2 == 0
                }),
            ]
        }
    }

    pub fn get_tiles(&self) -> &Vec<TileType> {
        &self.types
    }

    pub fn at(&self, idx: usize) -> &TileType {
        &self.types[idx]
    }
}
