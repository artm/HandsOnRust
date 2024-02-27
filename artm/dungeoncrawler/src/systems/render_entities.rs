use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
pub fn render_entities(ecs: &mut SubWorld, #[resource] camera: &mut Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_CHARACTERS);

    let fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .expect("There is always a player with field of view");

    let offset = Point::new(camera.fov.x1, camera.fov.y1);
    let mut entities = <(&Point, &Render)>::query();
    entities
        .iter(ecs)
        .filter(|(pos, _)| fov.can_see(pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}
