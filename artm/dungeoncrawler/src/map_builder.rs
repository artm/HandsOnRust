use crate::prelude::*;
use itertools::Itertools;

const MAX_NUM_CHAMBERS: usize = 20;
const MIN_CHAMBER_WIDTH: i32 = 5;
const MAX_CHAMBER_WIDTH: i32 = WORLD_WIDTH / 5;
const MIN_CHAMBER_HEIGHT: i32 = 5;
const MAX_CHAMBER_HEIGHT: i32 = WORLD_HEIGHT / 3;

pub struct MapBuilder {
    pub map: Map,
    pub chambers: Vec<Rect>,
    pub player_pos: Point,
    pub amulet_pos: Point,
}

impl MapBuilder {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            chambers: vec![],
            player_pos: Point::zero(),
            amulet_pos: Point::zero(),
        }
    }

    pub fn build(&mut self, rand: &mut RandomNumberGenerator) {
        self.fill(
            Rect::with_size(0, 0, WORLD_WIDTH, WORLD_HEIGHT),
            TileType::Wall,
        );

        while self.chambers.len() < MAX_NUM_CHAMBERS {
            let x = rand.range(1, WORLD_WIDTH - MIN_CHAMBER_WIDTH - 1);
            let y = rand.range(1, WORLD_HEIGHT - MIN_CHAMBER_HEIGHT - 1);
            let w = i32::min(
                WORLD_WIDTH - x - 2,
                rand.range(MIN_CHAMBER_WIDTH, MAX_CHAMBER_WIDTH),
            );
            let h = i32::min(
                WORLD_HEIGHT - y - 2,
                rand.range(MIN_CHAMBER_HEIGHT, MAX_CHAMBER_HEIGHT),
            );
            let r = Rect::with_size(x, y, w, h);
            let r1 = Rect::with_exact(r.x1 - 1, r.y1 - 1, r.x2 + 1, r.y2 + 1);
            if self.chambers.iter().any(|other| r1.intersect(other)) {
                continue;
            }
            self.chambers.push(r);
            self.fill(r, TileType::Floor);
        }
        self.chambers
            .sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (a, b) in self.chambers.iter().tuple_windows() {
            let (a, b) = (a.center(), b.center());
            let (x0, y0) = if rand.rand() { (a.x, b.y) } else { (b.x, a.y) };
            for y in i32::min(a.y, b.y)..=i32::max(a.y, b.y) {
                if let Some(i) = self.map.try_idx(Point::new(x0, y)) {
                    self.map.tiles[i] = TileType::Floor;
                }
            }
            for x in i32::min(a.x, b.x)..=i32::max(a.x, b.x) {
                if let Some(i) = self.map.try_idx(Point::new(x, y0)) {
                    self.map.tiles[i] = TileType::Floor;
                }
            }
        }
        self.player_pos = self.chambers[0].center();
        let dijkstra_map = DijkstraMap::new(
            WORLD_WIDTH,
            WORLD_HEIGHT,
            &[self.map.point_idx(self.player_pos)],
            &self.map,
            1000.0,
        );
        const UNREACHABLE: f32 = f32::MAX;
        self.amulet_pos = self.map.idx_point(
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
        );
    }

    fn fill(&mut self, rect: Rect, filler: TileType) {
        rect.for_each(|point| {
            if let Some(i) = self.map.try_idx(point) {
                self.map.tiles[i] = filler
            }
        });
    }
}
