use crate::prelude::*;

const DELTAS: [Point; 4] = [
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
];

#[system]
#[read_component(RandomWalk)]
#[read_component(Point)]
pub fn random_walk(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut rand = RandomNumberGenerator::new();
    <(Entity, &Point, &RandomWalk)>::query()
        .iter(ecs)
        .for_each(|(entity, pos, _)| {
            let delta = DELTAS[rand.range(0, 4)];
            commands.push((
                (),
                MotionIntent {
                    entity: *entity,
                    destination: *pos + delta,
                },
            ));
        });
}
