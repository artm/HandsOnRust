use bracket_lib::prelude::*;

enum GameMode {
    MainMenu,
    Playing,
    GameOver,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, NAVY, to_cp437('@'));
    }

    fn physics(&mut self) {
        self.velocity += 0.2;
        if self.velocity > 2.0 {
            self.velocity = 2.0;
        }
        self.x += 1;
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}
struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::MainMenu,
            player: Player::new(0, 25),
            frame_time: 0.0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon!");
        ctx.print_centered(7, "(P) Play");
        ctx.print_centered(9, "(Q) Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.reset(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > 75.0 {
            self.player.physics();
            self.frame_time = 0.0;
        }
        ctx.cls_bg(NAVY);
        self.player.render(ctx);

        if self.player.y >= 50 {
            self.mode = GameMode::GameOver;
        }
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                _ => {}
            }
        }
    }

    fn end_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over");
        ctx.print_centered(7, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.reset(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn reset(&mut self) {
        self.player = Player::new(5, 25);
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::MainMenu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::GameOver => self.end_menu(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
