use crate::Length;

impl Length {
    pub fn as_inches(&self) -> Self {
        match self {
            Self::In(_) => self.round(),
            Self::Ft(val) => Self::In(val * 12.).round(),
            Self::Yd(val) => Self::In(val * 36.).round(),
        }
    }
}
