use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn: &mut Turn) {
    *turn = match *turn {
        Turn::ExpectingInput => return,
        Turn::PlayerTurn => Turn::MonstersTurn,
        Turn::MonstersTurn => Turn::ExpectingInput,
    }
}
