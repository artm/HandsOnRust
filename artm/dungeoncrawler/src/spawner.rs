use crate::prelude::*;

const ENEMY_SPRITES: [char; 4] = ['E', 'O', 'o', 'g'];

pub fn spawn_player(world: &mut World, pos: Point) {
    world.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_enemy(world: &mut World, pos: Point, rand: &mut RandomNumberGenerator) {
    let glyph = to_cp437(
        *rand
            .random_slice_entry(&ENEMY_SPRITES)
            .expect("ENEMY_SPRITES isn't empty"),
    );

    world.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: glyph,
        },
    ));
}
