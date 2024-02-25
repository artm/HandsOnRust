#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Turn {
    ExpectingInput,
    Player,
    Enemies,
    GameOver,
}
