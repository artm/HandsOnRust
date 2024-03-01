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

    for y in y1..=y2 {
        for x in x1..=x2 {
            let pos = Point::new(x, y);
            if let Some(i) = map.try_idx(pos) {
                let tint = if fov.can_see(&pos) {
                    WHITE
                } else if map.seen[i] {
                    DARKGREY
                } else {
                    continue;
                };
                let tile = map.tiles[i];
                let glyph = to_cp437(match tile {
                    TileType::Wall => '#',
                    TileType::Floor => '.',
                });
                draw_batch.set(pos - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn render_demo_map(ecs: &SubWorld, #[resource] map: &Map) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_MAP);

    let fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .expect("There is always a player with field of view");

    for y in 0..WORLD_HEIGHT {
        for x in 0..WORLD_WIDTH {
            let pos = Point::new(x, y);
            if let Some(i) = map.try_idx(pos) {
                let tint = if fov.can_see(&pos) {
                    WHITE
                } else if map.seen[i] {
                    DARKGREY
                } else {
                    DARKGREEN
                };
                let tile = map.tiles[i];
                let glyph = to_cp437(match tile {
                    TileType::Wall => '#',
                    TileType::Floor => '.',
                });
                draw_batch.set(pos, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
