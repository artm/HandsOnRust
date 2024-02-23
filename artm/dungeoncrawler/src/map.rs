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

    pub fn map_idx(&self, pos: Point) -> usize {
        (pos.x + pos.y * WORLD_WIDTH) as usize
    }

    pub fn in_bounds(&self, pos: Point) -> bool {
        (0..WORLD_WIDTH).contains(&pos.x) && (0..WORLD_HEIGHT).contains(&pos.y)
    }

    pub fn can_enter(&self, pos: Point) -> bool {
        self.in_bounds(pos) && self.tiles[self.map_idx(pos)] == TileType::Floor
    }

    pub fn try_idx(&self, pos: Point) -> Option<usize> {
        if self.in_bounds(pos) {
            Some(self.map_idx(pos))
        } else {
            None
        }
    }
}
