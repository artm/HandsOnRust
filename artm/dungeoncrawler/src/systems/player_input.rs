use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
    #[resource] turn: &mut Turn,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Right => Point::new(1, 0),
            _ => Point::zero(),
        };
        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let newpos = *pos + delta;
                if map.can_enter(newpos.x, newpos.y) {
                    *pos = newpos;
                    camera.update(newpos);
                    *turn = Turn::PlayerTurn;
                }
            });
        }
    }
}
