use crate::Length;

impl Length {
    pub fn as_inches(&self) -> Self {
        match self {
            Self::In(_) => *self,
            Self::Ft(val) => Self::In(val * 12.),
            Self::Yrd(val) => Self::In(val * 36.),
        }
    }
}
