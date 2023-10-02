use crate::Length;

impl Length {
    pub fn as_feet(&self) -> Self {
        match self {
            Self::In(val) => Self::Ft(val / 12.),
            Self::Ft(_) => *self,
            Self::Yd(val) => Self::Ft(val * 3.),
            Self::Mi(val) => Self::Mi(val / 5280.),
        }
    }
}
