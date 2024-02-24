use crate::prelude::*;

#[system]
#[read_component(RandomWalk)]
#[read_component(Point)]
#[read_component(Player)]
pub fn random_walk(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut rand = RandomNumberGenerator::new();
    <(Entity, &Point, &RandomWalk)>::query()
        .iter(ecs)
        .for_each(|(entity, pos, _)| {
            let destination = *pos + DIRS[rand.range(0, 4)];
            let mut free = true;
            <(Entity, &Point)>::query()
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(victim, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((WantsToAttack {
                            attacker: *entity,
                            victim: *victim,
                        },));
                    }
                    free = false;
                });
            if free {
                commands.push((WantsToMove {
                    entity: *entity,
                    destination,
                },));
            }
        });
}
