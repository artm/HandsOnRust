use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn: &mut Turn,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Right => Point::new(1, 0),
            _ => Point::zero(),
        };
        <(Entity, &Point, &Player)>::query()
            .iter(ecs)
            .for_each(|(entity, pos, _)| {
                commands.push((
                    (),
                    MotionIntent {
                        entity: *entity,
                        destination: *pos + delta,
                    },
                ));
            });
        *turn = Turn::PlayerTurn;
    }
}
