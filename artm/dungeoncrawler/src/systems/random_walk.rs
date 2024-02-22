use crate::prelude::*;

const DELTAS: [Point; 4] = [
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
];

#[system]
#[read_component(RandomWalk)]
#[write_component(Point)]
pub fn random_walk(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut rand = RandomNumberGenerator::new();
    <(&mut Point, &RandomWalk)>::query()
        .iter_mut(ecs)
        .for_each(|(pos, _)| {
            let delta = DELTAS[rand.range(0, 4)];
            let newpos = *pos + delta;
            if map.can_enter(newpos.x, newpos.y) {
                *pos = newpos;
            }
        });
}
