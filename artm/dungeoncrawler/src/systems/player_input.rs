use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn: &mut Turn,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => DIR_LEFT,
            VirtualKeyCode::Right => DIR_RIGHT,
            VirtualKeyCode::Up => DIR_UP,
            VirtualKeyCode::Down => DIR_DOWN,
            _ => Point::zero(),
        };
        let (player_entity, destination) = <(Entity, &Point, &Player)>::query()
            .iter(ecs)
            .map(|(entity, pos, _)| (*entity, *pos + delta))
            .next()
            .expect("Player exists and has position");
        if delta.x != 0 || delta.y != 0 {
            let mut attacking = false;
            <(Entity, &Point)>::query()
                .filter(component::<Enemy>())
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    commands.push((WantsToAttack {
                        attacker: player_entity,
                        victim: *entity,
                    },));
                    attacking = true;
                });

            if !attacking {
                commands.push((WantsToMove {
                    entity: player_entity,
                    destination,
                },));
            }
        } else if let Ok(health) = ecs
            .entry_mut(player_entity)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current = i32::min(health.max, health.current + 1);
        }
        *turn = Turn::PlayerTurn;
    }
}
