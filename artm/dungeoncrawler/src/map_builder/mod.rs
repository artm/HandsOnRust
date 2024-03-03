mod cellular_automata;
mod drunkard;
mod empty;
mod rectrooms;

use crate::prelude::*;

trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(WORLD_WIDTH / 2, WORLD_HEIGHT / 2);
        map.tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == TileType::Floor)
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
            .filter(|(_, d)| *d > 10.0)
            .map(|(p, _)| p)
            .collect::<Vec<Point>>();
        let mut enemy_pos: Vec<Point> = Vec::new();
        for _ in 0..usize::min(50, tiles.len()) {
            let idx = rng.random_slice_index(&tiles).unwrap();
            enemy_pos.push(tiles.remove(idx));
        }
        enemy_pos
    }
}

pub struct MapBuilder {
    pub map: Map,
    pub chambers: Vec<Rect>,
    pub player_pos: Point,
    pub amulet_pos: Point,
    pub enemy_spawns: Vec<Point>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(rectrooms::RectroomArchitect {}),
            1 => Box::new(cellular_automata::CellularAutomataArchitect {}),
            _ => Box::new(drunkard::DrunkardArchitect {}),
        };
        architect.build(rng)
    }

    fn fill(&mut self, rect: Rect, filler: TileType) {
        rect.for_each(|point| {
            if let Some(i) = self.map.try_idx(point) {
                self.map.tiles[i] = filler
            }
        });
    }

    fn find_most_distant(&self, pos: Point) -> Point {
        let dijkstra_map = DijkstraMap::new(
            WORLD_WIDTH,
            WORLD_HEIGHT,
            &[self.map.point_idx(pos)],
            &self.map,
            1000.0,
        );
        const UNREACHABLE: f32 = f32::MAX;
        self.map.idx_point(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, d)| **d < UNREACHABLE)
                .max_by(|a, b| {
                    a.1.partial_cmp(b.1)
                        .expect("Two Dijkstra map entries have numeric distances")
                })
                .expect("There is a furtherst tile")
                .0,
        )
    }
}
