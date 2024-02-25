use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn: &mut Turn) {
    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .expect("There is always player with health");
    let is_dead = player_health.current <= 0;
    *turn = if is_dead {
        Turn::GameOver
    } else {
        match *turn {
            Turn::Player => Turn::Enemies,
            Turn::Enemies => Turn::ExpectingInput,
            _ => return,
        }
    }
}
