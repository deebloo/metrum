use crate::Length;

impl Length {
    pub fn as_yards(&self) -> Self {
        match self {
            Self::In(val) => Self::Yd(val / 36.).round(),
            Self::Ft(val) => Self::Yd(val / 3.).round(),
            Self::Yd(_) => self.round(),
        }
    }
}
