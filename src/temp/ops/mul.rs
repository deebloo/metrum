use crate::temp::Temp;
use std::ops::Mul;

impl Mul<f32> for Temp {
    type Output = Temp;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::C(val) => Self::C(val * rhs),
            Self::F(val) => Self::F(val * rhs),
            Self::K(val) => Self::K(val * rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_mul_with_f32() {
        let cc = Temp::C(10.) * 2.;
        let cf = Temp::F(10.) * 2.;
        let ck = Temp::K(10.) * 2.;

        assert_eq!(cc, Temp::C(20.));
        assert_eq!(cf, Temp::F(20.));
        assert_eq!(ck, Temp::K(20.));
    }
}
