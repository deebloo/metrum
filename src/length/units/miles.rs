use crate::Length;

impl Length {
    pub fn as_miles(&self) -> Self {
        match self {
            Self::In(val) => Self::Mi(val / 63360.),
            Self::Ft(val) => Self::Mi(val / 5280.),
            Self::Yd(val) => Self::Mi(val / 1760.),
            Self::Mi(_) => *self,
            Self::MM(val) => Self::Mi(val / 1.609e+6),
        }
    }
}
