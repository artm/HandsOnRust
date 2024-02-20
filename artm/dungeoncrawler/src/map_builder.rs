use crate::prelude::*;

const MAX_NUM_CHAMBERS: usize = 20;
const MIN_CHAMBER_WIDTH: i32 = 5;
const MAX_CHAMBER_WIDTH: i32 = SCREEN_WIDTH / 5;
const MIN_CHAMBER_HEIGHT: i32 = 5;
const MAX_CHAMBER_HEIGHT: i32 = SCREEN_HEIGHT / 3;

pub struct MapBuilder {
    pub map: Map,
    chambers: Vec<Rect>,
    pub player_pos: Point,
}

impl MapBuilder {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            chambers: vec![],
            player_pos: Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
        }
    }

    pub fn build(&mut self, rand: &mut RandomNumberGenerator) {
        self.fill(
            Rect::with_size(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT),
            TileType::Wall,
        );

        while self.chambers.len() < MAX_NUM_CHAMBERS {
            let x = rand.range(1, SCREEN_WIDTH - MIN_CHAMBER_WIDTH - 1);
            let y = rand.range(1, SCREEN_HEIGHT - MIN_CHAMBER_HEIGHT - 1);
            let w = i32::min(
                SCREEN_WIDTH - x - 2,
                rand.range(MIN_CHAMBER_WIDTH, MAX_CHAMBER_WIDTH),
            );
            let h = i32::min(
                SCREEN_HEIGHT - y - 2,
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
        self.player_pos = self.chambers[0].center();
    }

    fn fill(&mut self, rect: Rect, filler: TileType) {
        rect.for_each(|point| {
            if let Some(i) = self.map.try_idx(point.x, point.y) {
                self.map.tiles[i] = filler
            }
        });
    }
}
