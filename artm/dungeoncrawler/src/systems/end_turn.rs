use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn: &mut Turn) {
    let amulet_pos = <&Point>::query()
        .filter(component::<AmuletOfYala>())
        .iter(ecs)
        .next()
        .expect("There is always the amulet");
    let (player_health, player_pos) = <(&Health, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .expect("There is always player with health");
    *turn = if player_health.current <= 0 {
        Turn::GameOver
    } else if *amulet_pos == *player_pos {
        Turn::Victory
    } else {
        match *turn {
            Turn::Player => Turn::Enemies,
            Turn::Enemies => Turn::ExpectingInput,
            _ => return,
        }
    }
}
