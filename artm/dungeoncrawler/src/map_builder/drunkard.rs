use super::MapArchitect;
use crate::prelude::*;

const MAX_WALK_DISTANCE: usize = 400;
const TILES_COUNT: usize = WORLD_WIDTH as usize * WORLD_HEIGHT as usize;
const FLOOR_TARGET: usize = TILES_COUNT / 3;

pub struct DrunkardArchitect {}

impl MapArchitect for DrunkardArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            chambers: vec![],
            player_pos: Point::zero(),
            amulet_pos: Point::zero(),
            enemy_spawns: vec![],
        };

        mb.fill(
            Rect::with_size(0, 0, WORLD_WIDTH, WORLD_HEIGHT),
            TileType::Wall,
        );
        self.walk(rng, &mut mb.map);
        mb.player_pos = self.find_start(&mb.map);
        mb.amulet_pos = mb.find_most_distant(mb.player_pos);
        mb.enemy_spawns = self.place_enemies(rng, &mb.map, mb.player_pos);
        mb
    }
}

impl DrunkardArchitect {
    fn walk(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut start = Point::new(rng.range(0, WORLD_WIDTH), rng.range(0, WORLD_HEIGHT));
        loop {
            self.walk_once(start, rng, map);
            let floor: Vec<usize> = map
                .tiles
                .iter()
                .enumerate()
                .filter(|(_, tile)| **tile == TileType::Floor)
                .map(|(i, _)| i)
                .collect();
            if floor.len() >= FLOOR_TARGET {
                break;
            }
            let idx = *rng.random_slice_entry(&floor[..]).unwrap();
            start = map.idx_point(idx);
        }
    }

    fn walk_once(&mut self, start: Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut pos = start;
        for _ in 0..MAX_WALK_DISTANCE {
            if let Some(idx) = map.try_idx(pos) {
                map.tiles[idx] = TileType::Floor;
                pos += *rng.random_slice_entry(&DIRS).unwrap();
            } else {
                break;
            }
        }
    }
}
