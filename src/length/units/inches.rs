use crate::Length;

impl Length {
    pub fn as_inches(&self) -> Self {
        match self {
            Self::In(_) => self.round(),
            Self::Ft(val) => Self::In(val * 12.),
            Self::Yd(val) => Self::In(val * 36.),
            Self::Mi(val) => Self::Mi(val / 5280.),
        }
    }
}
