use crate::Length;

impl Length {
    pub fn as_feet(&self) -> Self {
        match self {
            Self::In(val) => Self::Ft(val * 12.),
            Self::Ft(_) => *self,
            Self::Yrd(val) => Self::Ft(val / 3.),
        }
    }
}
