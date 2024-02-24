use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn render_hud(ecs: &mut SubWorld) {
    let (_, health) = <(&Player, &Health)>::query()
        .iter(ecs)
        .next()
        .expect("There is always one player");

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_HUD);
    draw_batch.bar_horizontal(
        Point::zero(),
        HUD_WIDTH,
        health.current,
        health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_centered(0, format!(" Health: {}/{} ", health.current, health.max));
    draw_batch.submit(10000).expect("Batch error");
}
