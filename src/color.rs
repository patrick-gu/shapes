use std::fmt;

/// Colors that can be displayed as ANSI escape sequences.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub(crate) enum Color {
    #[allow(dead_code)]
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    Reset = 0,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[{}m", *self as u8)
    }
}
