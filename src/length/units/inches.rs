use crate::Length;

impl Length {
    pub fn as_inches(&self) -> Self {
        match self {
            Self::In(_) => *self,
            Self::Ft(val) => Self::In(val * 12.),
            Self::Yd(val) => Self::In(val * 36.),
            Self::Mi(val) => Self::In(val * 63360.),
            Self::MM(val) => Self::In(val / 25.4),
        }
    }
}
