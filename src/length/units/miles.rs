use crate::Length;

impl Length {
    pub fn as_miles(&self) -> Self {
        match self {
            Self::In(val) => Self::Yd(val / 36.),
            Self::Ft(val) => Self::Yd(val / 3.),
            Self::Yd(_) => *self,
            Self::Mi(_) => *self,
        }
    }
}
