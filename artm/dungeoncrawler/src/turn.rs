#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Turn {
    ExpectingInput,
    PlayerTurn,
    MonstersTurn,
}
