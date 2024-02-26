use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn render_map(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_MAP);

    let x1 = i32::max(0, camera.fov.x1);
    let x2 = i32::min(WORLD_WIDTH - 1, camera.fov.x2);
    let y1 = i32::max(0, camera.fov.y1);
    let y2 = i32::min(WORLD_HEIGHT - 1, camera.fov.y2);
    let offset = Point::new(camera.fov.x1, camera.fov.y1);

    let fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .expect("There is always a player with field of view");

    for x in x1..=x2 {
        for y in y1..=y2 {
            let pos = Point::new(x, y);
            if let Some(i) = map.try_idx(pos) {
                if !fov.points.contains(&pos) {
                    continue;
                }
                let tile = map.tiles[i];
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
