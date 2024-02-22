mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const WORLD_WIDTH: i32 = 80;
    pub const WORLD_HEIGHT: i32 = 80;
    pub const DISPLAY_WIDTH: i32 = 18;
    pub const DISPLAY_HEIGHT: i32 = 32;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}
use prelude::*;

struct State {
    world: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut rand = RandomNumberGenerator::new();
        let mut world = World::default();
        let mut map_builder = MapBuilder::new();
        map_builder.build(&mut rand);
        let mut resources = Resources::default();
        spawn_player(&mut world, map_builder.player_pos);
        map_builder
            .chambers
            .iter()
            .skip(1)
            .map(|chamber| chamber.center())
            .for_each(|pos| spawn_enemy(&mut world, pos, &mut rand));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_pos));
        Self {
            world,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.world, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_fullscreen(true)
        .build()?;
    main_loop(ctx, State::new())
}
