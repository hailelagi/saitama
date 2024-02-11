use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy)]
pub enum Marker {
    X,
    O,
    Empty,
}

impl Marker {
    pub fn new(mark: &str) -> Result<Self> {
        match mark {
            "X" => Ok(Marker::X),
            "O" => Ok(Marker::O),
            m => Err(anyhow!("did you mean X or O?: {}", m)),
        }
    }
}

impl PartialEq for Marker {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::X, Self::X) | (Self::O, Self::O) => true,
            _ => false,
        }
    }
}

impl Eq for Marker {}

impl std::fmt::Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Marker::O => write!(f, "({})", "O"),
            Marker::X => write!(f, "({})", "X"),
            Marker::Empty => write!(f, "({})", ""),
        }
    }
}
