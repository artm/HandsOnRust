mod camera;
mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const WORLD_WIDTH: i32 = 80;
    pub const WORLD_HEIGHT: i32 = 80;
    pub const DISPLAY_WIDTH: i32 = 40;
    pub const DISPLAY_HEIGHT: i32 = 25;
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}
use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut map_builder = MapBuilder::new();
        map_builder.build(&mut RandomNumberGenerator::new());
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_pos),
            camera: Camera::new(map_builder.player_pos),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.camera.update(self.player.pos);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple(DISPLAY_WIDTH, DISPLAY_HEIGHT)?
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(ctx, State::new())
}
