use dungeoncrawler::prelude::*;

struct State {
    world: World,
    resources: Resources,
    input_schedule: Schedule,
    player_schedule: Schedule,
    monsters_schedule: Schedule,
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
            input_schedule: build_input_scheduler(),
            player_schedule: build_player_scheduler(),
            monsters_schedule: build_monsters_scheduler(),
        }
    }

    fn restart(&mut self) {
        // FIXME dont repeat myself
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
        self.world = world;
        self.resources = resources;
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_HUD);
        ctx.print_color_centered(5, WHITE, BLACK, "Game Over");
        ctx.print_color_centered(7, RED, BLACK, "Slain before achieving objective");
        ctx.print_color_centered(9, ORANGE, BLACK, "Better luck next time");
        ctx.print_color_centered(11, GREEN, BLACK, "(R)eincarnate");
        ctx.print_color_centered(13, GREY50, BLACK, "(Q)uit trying");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R => self.restart(),
                VirtualKeyCode::Q => ctx.quit(),
                _ => (),
            }
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_HUD);
        ctx.print_color_centered(7, WHITE, BLACK, "Rejoyce!");
        ctx.print_color_centered(9, ORANGE, BLACK, "The Amulet of Yala is yours at last!");
        ctx.print_color_centered(11, GREEN, BLACK, "(R)eincarnate");
        ctx.print_color_centered(13, GREY50, BLACK, "(Q)uit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R => self.restart(),
                VirtualKeyCode::Q => ctx.quit(),
                _ => (),
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        for i in 0..LAYER_COUNT {
            ctx.set_active_console(i);
            ctx.cls();
        }
        ctx.set_active_console(LAYER_MAP);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        self.resources.insert(ctx.key);
        let turn = *self.resources.get::<Turn>().unwrap();
        match turn {
            Turn::ExpectingInput => self
                .input_schedule
                .execute(&mut self.world, &mut self.resources),
            Turn::Player => self
                .player_schedule
                .execute(&mut self.world, &mut self.resources),
            Turn::Enemies => self
                .monsters_schedule
                .execute(&mut self.world, &mut self.resources),
            Turn::GameOver => self.game_over(ctx),
            Turn::Victory => self.victory(ctx),
        }
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
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(HUD_WIDTH, HUD_HEIGHT, "terminal8x8.png")
        .with_fullscreen(true)
        .build()?;
    main_loop(ctx, State::new())
}
