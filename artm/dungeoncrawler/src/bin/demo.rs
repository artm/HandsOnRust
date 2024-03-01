use dungeoncrawler::prelude::*;

struct State {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl State {
    fn new() -> Self {
        let mut rand = RandomNumberGenerator::new();
        let mut world = World::default();
        let map_builder = MapBuilder::new(&mut rand);
        let mut resources = Resources::default();
        spawn_player(&mut world, map_builder.player_pos);
        spawn_amulet_of_yala(&mut world, map_builder.amulet_pos);
        map_builder
            .enemy_spawns
            .iter()
            .for_each(|pos| spawn_enemy(&mut world, *pos, &mut rand));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_pos));
        resources.insert(Turn::ExpectingInput);
        Self {
            world,
            resources,
            schedule: build_demo_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        for i in 0..2 {
            ctx.set_active_console(i);
            ctx.cls();
        }
        ctx.set_active_console(LAYER_MAP);
        self.resources.insert(ctx.key);
        self.schedule.execute(&mut self.world, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_dimensions(WORLD_WIDTH, WORLD_HEIGHT)
        .with_tile_dimensions(16, 16)
        .with_title("Dungeon Crawler Demo")
        .with_fps_cap(30.0)
        .with_resource_path("resources/")
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(WORLD_WIDTH, WORLD_HEIGHT, "terminal8x8.png")
        .with_simple_console_no_bg(WORLD_WIDTH, WORLD_HEIGHT, "terminal8x8.png")
        .build()?;
    main_loop(ctx, State::new())
}
