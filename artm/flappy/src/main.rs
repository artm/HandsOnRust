use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 24;
const FRAME_DURATION: f32 = 80.0;
const FLAP_VELOCITY: f32 = -1.0;
const TERMINAL_VELOCITY: f32 = 2.0;
const GRAVITY_DV: f32 = 0.2;
const MIN_GAP: i32 = 2;
const MAX_GAP: i32 = 18;

enum GameMode {
    MainMenu,
    Playing,
    GameOver,
}

struct Player {
    x: i32,
    y: f32,
    velocity: f32,
    frame: usize,
}

impl Player {
    const FRAMES: [u16; 4] = [64, 65, 66, 65];

    fn new(x: i32, y: f32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
            frame: 0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(0.0, self.y),
            0,
            Degrees::new(0.0),
            PointF::new(1.0, 1.0),
            WHEAT,
            NAVY,
            Self::FRAMES[self.frame],
        );
        ctx.set_active_console(0);
    }

    fn physics(&mut self) {
        self.velocity += GRAVITY_DV;
        if self.velocity > TERMINAL_VELOCITY {
            self.velocity = TERMINAL_VELOCITY;
        }
        self.x += 1;
        self.frame = (self.frame + 1) % Self::FRAMES.len();
        self.y += self.velocity;
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }

    fn flap(&mut self) {
        self.velocity = FLAP_VELOCITY;
    }
}

struct Obstacle {
    x: i32,
    h: i32,
    gap: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        let gap = i32::max(MIN_GAP, MAX_GAP - score);
        let h = random.range(2, SCREEN_HEIGHT - gap - 2);
        Self { x, h, gap }
    }

    fn render(&mut self, ctx: &mut BTerm, player: &Player) {
        let x = self.x - player.x;
        for y in 0..self.h {
            ctx.set(x, y, WHITE, RED, to_cp437(' '));
        }
        for y in self.h + self.gap..SCREEN_HEIGHT {
            ctx.set(x, y, WHITE, RED, to_cp437(' '));
        }
    }

    fn collides(&self, player: &Player) -> bool {
        let py = player.y as i32;
        self.x == player.x && (py < self.h || py >= self.h + self.gap)
    }
}

struct State {
    mode: GameMode,
    player: Player,
    obstacle: Obstacle,
    frame_time: f32,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::MainMenu,
            player: Player::new(0, SCREEN_HEIGHT as f32 / 2.0),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            frame_time: 0.0,
            score: 0,
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
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.player.physics();
            self.frame_time = 0.0;
        }
        self.obstacle.render(ctx, &self.player);
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, format!("Score: {}", self.score));
        if let Some(key) = ctx.key {
            if key == VirtualKeyCode::Space {
                self.player.flap();
            }
        }
        if self.obstacle.x < self.player.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y >= SCREEN_HEIGHT as f32 || self.obstacle.collides(&self.player) {
            self.mode = GameMode::GameOver;
            ctx.set_active_console(1);
            ctx.cls();
            ctx.set_active_console(0);
        }
    }

    fn end_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over");
        ctx.print_centered(7, format!("Score: {}", self.score));
        ctx.print_centered(9, "(P) Play Again");
        ctx.print_centered(11, "(Q) Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.reset(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn reset(&mut self) {
        self.player = Player::new(0, SCREEN_HEIGHT as f32 / 2.0);
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
    let font = "../resources/16x16_sm_ascii.png";
    let context = BTermBuilder::new()
        .with_font(font, 16, 16)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, font)
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, font)
        .with_title("Flappy Dragon")
        .with_tile_dimensions(16, 16)
        .build()?;
    main_loop(context, State::new())
}
