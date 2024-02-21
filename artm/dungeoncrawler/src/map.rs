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

    pub fn map_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % WORLD_WIDTH, idx as i32 / WORLD_WIDTH)
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        let x1 = i32::max(0, camera.fov.x1);
        let x2 = i32::min(WORLD_WIDTH - 1, camera.fov.x2);
        let y1 = i32::max(0, camera.fov.y1);
        let y2 = i32::min(WORLD_HEIGHT - 1, camera.fov.y2);

        for x in x1..=x2 {
            for y in y1..=y2 {
                if let Some(i) = self.try_idx(x, y) {
                    let tile = self.tiles[i];
                    match tile {
                        TileType::Wall => ctx.set(
                            x - camera.fov.x1,
                            y - camera.fov.y1,
                            WHITE,
                            BLACK,
                            to_cp437('#'),
                        ),
                        TileType::Floor => ctx.set(
                            x - camera.fov.x1,
                            y - camera.fov.y1,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                    }
                }
            }
        }
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
