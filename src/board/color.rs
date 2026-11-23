//! Side to move / piece ownership.

/// White or Black.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 1,
        }
    }
}
