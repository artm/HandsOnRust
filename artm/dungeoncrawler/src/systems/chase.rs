use crate::prelude::*;

#[system]
#[read_component(ChasingPlayer)]
#[read_component(Point)]
#[read_component(Player)]
pub fn chase(ecs: &mut SubWorld, #[resource] map: &Map, commands: &mut CommandBuffer) {
    let mut player = <(&Point, &Player)>::query();
    let mut chasers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point)>::query();

    let player_pos = *player.iter(ecs).next().unwrap().0;
    let player_idx = map.point_idx(player_pos);
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(WORLD_WIDTH, WORLD_HEIGHT, &search_targets, map, 1000.0);

    chasers.iter(ecs).for_each(|(entity, pos, _)| {
        if let Some(chase_pos_idx) =
            DijkstraMap::find_lowest_exit(&dijkstra_map, map.point_idx(*pos), map)
        {
            let next_to_player = DistanceAlg::Pythagoras.distance2d(*pos, player_pos) < 1.2;
            let destination = if next_to_player {
                player_pos
            } else {
                map.idx_point(chase_pos_idx)
            };

            let mut free = true;
            positions
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(dest_entity, _)| {
                    let dest_is_player = ecs
                        .entry_ref(*dest_entity)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok();
                    if dest_is_player {
                        commands.push((WantsToAttack {
                            attacker: *entity,
                            victim: *dest_entity,
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
        }
    });
}
