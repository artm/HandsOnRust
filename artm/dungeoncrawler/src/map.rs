use crate::prelude::*;

const NUM_TILES: usize = (WORLD_WIDTH * WORLD_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub seen: Vec<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            seen: vec![false; NUM_TILES],
        }
    }

    pub fn point_idx(&self, pos: Point) -> usize {
        (pos.x + pos.y * WORLD_WIDTH) as usize
    }

    pub fn idx_point(&self, idx: usize) -> Point {
        Point::new(idx % WORLD_WIDTH as usize, idx / WORLD_WIDTH as usize)
    }

    pub fn in_bounds(&self, pos: Point) -> bool {
        (0..WORLD_WIDTH).contains(&pos.x) && (0..WORLD_HEIGHT).contains(&pos.y)
    }

    pub fn can_enter(&self, pos: Point) -> bool {
        self.in_bounds(pos) && self.tiles[self.point_idx(pos)] == TileType::Floor
    }

    pub fn try_idx(&self, pos: Point) -> Option<usize> {
        if self.in_bounds(pos) {
            Some(self.point_idx(pos))
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let pos = self.idx_point(idx);
        DIRS.iter()
            .map(|delta| pos + *delta)
            .filter(|dest| self.can_enter(*dest))
            .map(|dest| (self.point_idx(dest), 1.0))
            .collect()
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.idx_point(idx1), self.idx_point(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] != TileType::Floor
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(WORLD_WIDTH, WORLD_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
    }
}
