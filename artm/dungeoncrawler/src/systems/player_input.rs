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
            VirtualKeyCode::Left => DIR_LEFT,
            VirtualKeyCode::Right => DIR_RIGHT,
            VirtualKeyCode::Up => DIR_UP,
            VirtualKeyCode::Down => DIR_DOWN,
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
