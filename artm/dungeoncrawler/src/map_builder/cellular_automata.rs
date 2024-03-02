use itertools::Itertools;

use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect {}

const OFFSETS: [i32; 8] = [
    -WORLD_WIDTH - 1,
    -WORLD_WIDTH,
    -WORLD_WIDTH + 1,
    -1,
    1,
    WORLD_WIDTH - 1,
    WORLD_WIDTH,
    WORLD_WIDTH + 1,
];

impl CellularAutomataArchitect {
    fn random_fill(&self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        for i in 0..map.tiles.len() {
            map.tiles[i] = if rng.range(0, 100) < 55 {
                TileType::Floor
            } else {
                TileType::Wall
            }
        }
    }

    fn count_neighbours(&self, tiles: &Vec<TileType>, idx: usize) -> usize {
        OFFSETS
            .iter()
            .map(|offset| idx as i32 + offset)
            .filter(|idx| {
                *idx >= 0 && *idx < tiles.len() as i32 && tiles[*idx as usize] == TileType::Floor
            })
            .count()
    }

    fn conway(&self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut next = map.tiles.clone();
        for i in 0..map.tiles.len() {
            let cnt = self.count_neighbours(&map.tiles, i);
            next[i] = if cnt > 0 && cnt < 5 {
                TileType::Floor
            } else {
                TileType::Wall
            }
        }
        map.tiles = next;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(WORLD_WIDTH / 2, WORLD_HEIGHT / 2);
        map.tiles
            .iter()
            .enumerate()
            .filter(|(i, tile)| **tile == TileType::Floor)
            .map(|(i, _)| {
                let p = map.idx_point(i);
                (p, DistanceAlg::Pythagoras.distance2d(p, center))
            })
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(p, _)| p)
            .unwrap()
    }

    fn place_enemies(
        &self,
        rng: &mut RandomNumberGenerator,
        map: &Map,
        player_pos: Point,
    ) -> Vec<Point> {
        let mut tiles = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == TileType::Floor)
            .map(|(idx, _)| {
                let p = map.idx_point(idx);
                (p, DistanceAlg::Pythagoras.distance2d(p, player_pos))
            })
            .filter(|(p, d)| *d > 10.0)
            .map(|(p, _)| p)
            .collect::<Vec<Point>>();
        let mut enemy_pos: Vec<Point> = Vec::new();
        for i in 0..50 {
            let idx = rng.random_slice_index(&tiles).unwrap();
            enemy_pos.push(tiles.remove(idx));
        }
        enemy_pos
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            chambers: vec![],
            player_pos: Point::zero(),
            amulet_pos: Point::zero(),
            enemy_spawns: vec![],
        };

        self.random_fill(rng, &mut mb.map);
        for _ in 0..10 {
            self.conway(rng, &mut mb.map);
        }
        mb.player_pos = self.find_start(&mb.map);
        mb.amulet_pos = mb.find_most_distant(mb.player_pos);
        mb.enemy_spawns = self.place_enemies(rng, &mb.map, mb.player_pos);
        mb
    }
}
