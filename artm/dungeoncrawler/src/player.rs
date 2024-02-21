use crate::prelude::*;

pub struct Player {
    pub pos: Point,
}

impl Player {
    pub fn new(pos: Point) -> Self {
        Self { pos }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set(
            self.pos.x - camera.fov.x1,
            self.pos.y - camera.fov.y1,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Down => Point::new(0, 1),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Right => Point::new(1, 0),
                _ => Point::zero(),
            };
            let newpos = self.pos + delta;
            if map.can_enter(newpos.x, newpos.y) {
                self.pos = newpos;
            }
        }
    }
}
