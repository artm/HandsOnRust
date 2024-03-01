mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const WORLD_WIDTH: i32 = 80;
    pub const WORLD_HEIGHT: i32 = 80;
    pub const DISPLAY_WIDTH: i32 = 18;
    pub const DISPLAY_HEIGHT: i32 = 32;
    pub const HUD_WIDTH: i32 = DISPLAY_WIDTH * 4;
    pub const HUD_HEIGHT: i32 = DISPLAY_HEIGHT * 4;
    pub const DIR_LEFT: Point = Point { x: -1, y: 0 };
    pub const DIR_RIGHT: Point = Point { x: 1, y: 0 };
    pub const DIR_UP: Point = Point { x: 0, y: -1 };
    pub const DIR_DOWN: Point = Point { x: 0, y: 1 };
    pub const DIRS: [Point; 4] = [DIR_LEFT, DIR_RIGHT, DIR_UP, DIR_DOWN];
    pub const LAYER_MAP: usize = 0;
    pub const LAYER_CHARACTERS: usize = 1;
    pub const LAYER_HUD: usize = 2;
    pub const LAYER_COUNT: usize = 3;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn::*;
}
