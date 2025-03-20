use super::Speed;
use std::ops::Div;

impl Div<f64> for Speed {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            meters_per_second: self.meters_per_second / rhs,
        }
    }
}

impl Div for Speed {
    type Output = f64;

    fn div(self, rhs: Self) -> f64 {
        self.meters_per_second / rhs.meters_per_second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_same_unit() {
        let s1 = Speed::from_mps(6.0);
        let s2 = Speed::from_mps(2.0);
        let quotient = s1 / s2;
        assert_eq!(quotient, 3.0);
    }

    #[test]
    fn test_div_different_units() {
        let s1 = Speed::from_mps(1.0);
        let s2 = Speed::from_kmph(3.6);
        let quotient = s1 / s2;
        assert_eq!(quotient, 1.0);
    }

    #[test]
    fn test_div_by_scalar() {
        let s = Speed::from_mps(6.0);
        let quotient = s / 2.0;
        assert_eq!(quotient.as_mps(), 3.0);
    }

    #[test]
    fn test_div_anticommutative() {
        let s1 = Speed::from_mps(1.0);
        let s2 = Speed::from_kmph(3.6);
        let quotient1 = s1 / s2;
        let quotient2 = 1.0 / (s2 / s1);
        assert_eq!(quotient1, quotient2);
    }
}
