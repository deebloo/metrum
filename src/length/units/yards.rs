use crate::Length;

impl Length {
    pub fn as_yards(&self) -> Self {
        match self {
            Self::In(val) => Self::Yd(val / 36.),
            Self::Ft(val) => Self::Yd(val / 3.),
            Self::Yd(_) => *self,
            Self::Mi(val) => Self::Yd(val * 1760.),
        }
    }
}
