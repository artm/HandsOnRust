mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}
use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut map_builder = MapBuilder::new();
        map_builder.build(&mut RandomNumberGenerator::new());
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_pos),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
        self.player.update(ctx, &self.map);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(25.0)
        .build()?;
    main_loop(ctx, State::new())
}
