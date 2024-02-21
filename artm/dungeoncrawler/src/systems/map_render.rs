use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let x1 = i32::max(0, camera.fov.x1);
    let x2 = i32::min(WORLD_WIDTH - 1, camera.fov.x2);
    let y1 = i32::max(0, camera.fov.y1);
    let y2 = i32::min(WORLD_HEIGHT - 1, camera.fov.y2);
    let offset = Point::new(camera.fov.x1, camera.fov.y1);

    for x in x1..=x2 {
        for y in y1..=y2 {
            if let Some(i) = map.try_idx(x, y) {
                let tile = map.tiles[i];
                let pos = Point::new(x, y);

                let glyph = match tile {
                    TileType::Wall => to_cp437('#'),
                    TileType::Floor => to_cp437('.'),
                };
                draw_batch.set(pos - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
