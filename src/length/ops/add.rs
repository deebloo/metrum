use crate::length::Length;
use std::ops::Add;

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::In(val) => {
                let target: f64 = rhs.as_inches().into();

                Self::In(val + target)
            }
            Self::Ft(val) => {
                let target: f64 = rhs.as_feet().into();

                Self::Ft(val + target)
            }
            Self::Yd(val) => {
                let target: f64 = rhs.as_yards().into();

                Self::Yd(val + target)
            }
            Self::Mi(val) => {
                let target: f64 = rhs.as_miles().into();

                Self::Mi(val + target)
            }
            Self::MM(val) => {
                let target: f64 = rhs.as_milimeters().into();

                Self::MM(val + target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_to_in() {
        assert_eq!(Length::In(36.) + Length::In(36.), Length::In(72.));
        assert_eq!(Length::In(36.) + Length::Ft(3.), Length::In(72.));
        assert_eq!(Length::In(36.) + Length::Yd(1.), Length::In(72.));
    }
}
