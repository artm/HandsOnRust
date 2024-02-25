use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn: &mut Turn) {
    *turn = match *turn {
        Turn::ExpectingInput => return,
        Turn::Player => Turn::Enemies,
        Turn::Enemies => Turn::ExpectingInput,
    }
}
