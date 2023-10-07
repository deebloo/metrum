use crate::Length;

impl Length {
    pub fn as_milimeters(&self) -> Self {
        match self {
            Self::In(val) => Self::MM(val * 25.4),
            Self::Ft(val) => Self::MM(val * 304.8),
            Self::Yd(val) => Self::MM(val * 914.4),
            Self::Mi(val) => Self::MM(val * 1.609e+6),
            Self::MM(_) => *self,
        }
    }
}
