use crate::length::Length;
use std::ops::Add;

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::In(val) => {
                let target: f32 = rhs.as_inches().into();

                Self::In(val + target)
            }
            Self::Ft(val) => {
                let target: f32 = rhs.as_feet().into();

                Self::Ft(val + target)
            }
            Self::Yd(val) => {
                let target: f32 = rhs.as_yards().into();

                Self::Yd(val + target)
            }
        }
    }
}

#[cfg(test)]
mod tests {}
