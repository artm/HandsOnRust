use super::MapArchitect;
use crate::prelude::*;
use itertools::Itertools;

const MAX_NUM_CHAMBERS: usize = 20;
const MIN_CHAMBER_WIDTH: i32 = 5;
const MAX_CHAMBER_WIDTH: i32 = WORLD_WIDTH / 5;
const MIN_CHAMBER_HEIGHT: i32 = 5;
const MAX_CHAMBER_HEIGHT: i32 = WORLD_HEIGHT / 3;

pub struct RectroomArchitect {}

impl MapArchitect for RectroomArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
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

        while mb.chambers.len() < MAX_NUM_CHAMBERS {
            let x = rng.range(1, WORLD_WIDTH - MIN_CHAMBER_WIDTH - 1);
            let y = rng.range(1, WORLD_HEIGHT - MIN_CHAMBER_HEIGHT - 1);
            let w = i32::min(
                WORLD_WIDTH - x - 2,
                rng.range(MIN_CHAMBER_WIDTH, MAX_CHAMBER_WIDTH),
            );
            let h = i32::min(
                WORLD_HEIGHT - y - 2,
                rng.range(MIN_CHAMBER_HEIGHT, MAX_CHAMBER_HEIGHT),
            );
            let r = Rect::with_size(x, y, w, h);
            let r1 = Rect::with_exact(r.x1 - 1, r.y1 - 1, r.x2 + 1, r.y2 + 1);
            if mb.chambers.iter().any(|other| r1.intersect(other)) {
                continue;
            }
            mb.chambers.push(r);
            mb.fill(r, TileType::Floor);
        }
        mb.chambers.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (a, b) in mb.chambers.iter().tuple_windows() {
            let (a, b) = (a.center(), b.center());
            let (x0, y0) = if rng.rand() { (a.x, b.y) } else { (b.x, a.y) };
            for y in i32::min(a.y, b.y)..=i32::max(a.y, b.y) {
                if let Some(i) = mb.map.try_idx(Point::new(x0, y)) {
                    mb.map.tiles[i] = TileType::Floor;
                }
            }
            for x in i32::min(a.x, b.x)..=i32::max(a.x, b.x) {
                if let Some(i) = mb.map.try_idx(Point::new(x, y0)) {
                    mb.map.tiles[i] = TileType::Floor;
                }
            }
        }
        mb.player_pos = mb.chambers[0].center();
        mb.amulet_pos = mb.find_most_distant(mb.player_pos);
        for chamber in mb.chambers.iter().skip(1) {
            mb.enemy_spawns.push(chamber.center());
        }
        mb
    }
}
