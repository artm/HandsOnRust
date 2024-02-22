use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|pos| player_pos = *pos);
    <(&Point, Entity)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(pos, _)| **pos == player_pos)
        .for_each(|(_, entity)| commands.remove(*entity));
}
