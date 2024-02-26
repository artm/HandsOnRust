use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct ProtoMonster {
    name: &'static str,
    max_health: i32,
    glyph: char,
}

const PROTO_MONSTERS: [ProtoMonster; 4] = [
    ProtoMonster {
        name: "Eton",
        max_health: 8,
        glyph: 'E',
    },
    ProtoMonster {
        name: "Ogre",
        max_health: 4,
        glyph: 'O',
    },
    ProtoMonster {
        name: "Orc",
        max_health: 2,
        glyph: 'o',
    },
    ProtoMonster {
        name: "Goblin",
        max_health: 1,
        glyph: 'g',
    },
];

pub fn spawn_player(world: &mut World, pos: Point) {
    world.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
        FieldOfView::new(8),
    ));
}

pub fn spawn_enemy(world: &mut World, pos: Point, rand: &mut RandomNumberGenerator) {
    let proto = rand
        .random_slice_entry(&PROTO_MONSTERS)
        .expect("PROTO_MONSTERS isn't empty")
        .clone();

    world.push((
        Enemy,
        ChasingPlayer,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(proto.glyph),
        },
        Name(proto.name.into()),
        Health {
            current: proto.max_health,
            max: proto.max_health,
        },
        FieldOfView::new(6),
    ));
}

pub fn spawn_amulet_of_yala(world: &mut World, pos: Point) {
    world.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
    ));
}
