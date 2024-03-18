#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    OutOfBounds,
    AlreadyOpened,
    Lose,
    Win,
}
