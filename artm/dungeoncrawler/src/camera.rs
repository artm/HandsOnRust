use crate::prelude::*;

pub struct Camera {
    pub fov: Rect,
}

impl Camera {
    pub fn new(center: Point) -> Self {
        Self {
            fov: Rect::with_exact(
                center.x - DISPLAY_WIDTH / 2,
                center.y - DISPLAY_HEIGHT / 2,
                center.x + DISPLAY_WIDTH / 2,
                center.y + DISPLAY_HEIGHT / 2,
            ),
        }
    }

    pub fn update(&mut self, center: Point) {
        self.fov = Rect::with_exact(
            center.x - DISPLAY_WIDTH / 2,
            center.y - DISPLAY_HEIGHT / 2,
            center.x + DISPLAY_WIDTH / 2,
            center.y + DISPLAY_HEIGHT / 2,
        );
    }
}
