use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

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
        (x + y * SCREEN_WIDTH) as usize
    }

    pub fn map_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % SCREEN_WIDTH, idx as i32 / SCREEN_WIDTH)
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let (x, y) = self.map_xy(i);
            match tile {
                TileType::Wall => ctx.set(x, y, WHITE, BROWN1, to_cp437('#')),
                TileType::Floor => ctx.set(x, y, WHEAT, BLACK, to_cp437('.')),
            }
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < SCREEN_WIDTH && y >= 0 && y < SCREEN_HEIGHT
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
