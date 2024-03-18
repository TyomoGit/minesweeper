use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Bomb,
    Safe(u8),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Bomb => write!(f, "@"),
            Cell::Safe(n_bombs) => write!(f, "{}", n_bombs),
        }
    }
}
