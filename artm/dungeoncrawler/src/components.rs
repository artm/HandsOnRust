use std::collections::HashSet;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RandomWalk;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub points: HashSet<Point>,
    pub radius: i32,
    pub dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            points: HashSet::new(),
            radius,
            dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            points: HashSet::new(),
            radius: self.radius,
            dirty: true,
        }
    }

    pub fn can_see(&self, pos: &Point) -> bool {
        self.points.contains(pos)
    }
}
