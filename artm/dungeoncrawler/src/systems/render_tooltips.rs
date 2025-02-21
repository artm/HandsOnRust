use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Name)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn render_tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let offset = Point::new(camera.fov.x1, camera.fov.y1);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_HUD);
    let fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .expect("There is always a player with field of view");
    <(Entity, &Point, &Name)>::query()
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos && fov.can_see(pos))
        .for_each(|(entity, _, name)| {
            let hud_pos = *mouse_pos * 4 + Point::new(2, -2);
            let tip = if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>()
            {
                format!("{}: {}/{}", name.0.clone(), health.current, health.max)
            } else {
                name.0.clone()
            };
            draw_batch.print_centered_at(hud_pos, tip);
        });

    draw_batch.submit(10100).expect("Batch error");
}
