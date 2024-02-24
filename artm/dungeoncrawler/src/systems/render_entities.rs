use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn render_entities(ecs: &mut SubWorld, #[resource] camera: &mut Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_CHARACTERS);

    let offset = Point::new(camera.fov.x1, camera.fov.y1);
    let mut entities = <(&Point, &Render)>::query();
    for (pos, render) in entities.iter(ecs) {
        draw_batch.set(*pos - offset, render.color, render.glyph);
    }

    draw_batch.submit(5000).expect("Batch error");
}
