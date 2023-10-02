use crate::length::Length;
use std::ops::Sub;

impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::In(val) => {
                let target: f64 = rhs.as_inches().into();

                Self::In(val - target)
            }
            Self::Ft(val) => {
                let target: f64 = rhs.as_feet().into();

                Self::Ft(val - target)
            }
            Self::Yd(val) => {
                let target: f64 = rhs.as_yards().into();

                Self::Yd(val - target)
            }
            Self::Mi(val) => {
                let target: f64 = rhs.as_miles().into();

                Self::Mi(val - target)
            }
        }
    }
}
