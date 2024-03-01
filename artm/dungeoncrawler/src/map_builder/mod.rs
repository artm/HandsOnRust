mod empty;
mod rectrooms;

use crate::prelude::*;

use self::rectrooms::RectroomArchitect;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
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
        let mut architect = RectroomArchitect {};
        architect.new(rng)
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
