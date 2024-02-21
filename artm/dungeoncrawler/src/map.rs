use crate::prelude::*;

const NUM_TILES: usize = (WORLD_WIDTH * WORLD_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn map_idx(&self, x: i32, y: i32) -> usize {
        (x + y * WORLD_WIDTH) as usize
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0..WORLD_WIDTH).contains(&x) && (0..WORLD_HEIGHT).contains(&y)
    }

    pub fn can_enter(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.tiles[self.map_idx(x, y)] == TileType::Floor
    }

    pub fn try_idx(&self, x: i32, y: i32) -> Option<usize> {
        if self.in_bounds(x, y) {
            Some(self.map_idx(x, y))
        } else {
            None
        }
    }
}
