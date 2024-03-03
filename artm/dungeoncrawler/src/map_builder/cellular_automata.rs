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

impl MapArchitect for CellularAutomataArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            chambers: vec![],
            player_pos: Point::zero(),
            amulet_pos: Point::zero(),
            enemy_spawns: vec![],
        };

        self.random_fill(rng, &mut mb.map);
        for _ in 0..10 {
            self.conway(&mut mb.map);
        }
        mb.player_pos = self.find_start(&mb.map);
        mb.amulet_pos = mb.find_most_distant(mb.player_pos);
        mb.enemy_spawns = self.place_enemies(rng, &mb.map, mb.player_pos);
        mb
    }
}

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

    fn conway(&self, map: &mut Map) {
        let mut next: Vec<TileType> = Vec::new();
        for i in 0..map.tiles.len() {
            let cnt = self.count_neighbours(&map.tiles, i);
            next.push(if cnt > 0 && cnt < 5 {
                TileType::Floor
            } else {
                TileType::Wall
            });
        }
        map.tiles = next;
    }
}
