use crate::prelude::*;

#[system]
#[read_component(RandomWalk)]
#[read_component(Point)]
pub fn random_walk(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut rand = RandomNumberGenerator::new();
    <(Entity, &Point, &RandomWalk)>::query()
        .iter(ecs)
        .for_each(|(entity, pos, _)| {
            let delta = DIRS[rand.range(0, 4)];
            commands.push((
                (),
                MotionIntent {
                    entity: *entity,
                    destination: *pos + delta,
                },
            ));
        });
}
