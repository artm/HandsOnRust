use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
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
            TileType::Floor,
        );
        mb.player_pos = Point::new(WORLD_WIDTH / 2, WORLD_HEIGHT / 2);
        mb.amulet_pos = mb.find_most_distant(mb.player_pos);
        for _ in 0..50 {
            mb.enemy_spawns.push(Point::new(
                rng.range(0, WORLD_WIDTH),
                rng.range(0, WORLD_HEIGHT),
            ));
        }
        mb
    }
}
