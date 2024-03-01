mod chase;
mod combat;
mod end_turn;
mod motion;
mod player_input;
mod render_entities;
mod render_hud;
mod render_map;
mod render_tooltips;
mod vision;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(vision::vision_system())
        .flush()
        .add_system(render_map::render_map_system())
        .add_system(render_entities::render_entities_system())
        .add_system(render_hud::render_hud_system())
        .add_system(render_tooltips::render_tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat::combat_system())
        .add_system(motion::motion_system())
        .flush()
        .add_system(vision::vision_system())
        .flush()
        .add_system(render_map::render_map_system())
        .add_system(render_entities::render_entities_system())
        .add_system(render_hud::render_hud_system())
        .add_system(render_tooltips::render_tooltips_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monsters_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(chase::chase_system())
        .flush()
        .add_system(combat::combat_system())
        .add_system(motion::motion_system())
        .flush()
        .add_system(vision::vision_system())
        .flush()
        .add_system(render_map::render_map_system())
        .add_system(render_entities::render_entities_system())
        .add_system(render_hud::render_hud_system())
        .add_system(render_tooltips::render_tooltips_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_demo_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(motion::motion_system())
        .flush()
        .add_system(vision::vision_system())
        .flush()
        .add_system(render_map::render_demo_map_system())
        .add_system(render_entities::render_demo_entities_system())
        .build()
}
