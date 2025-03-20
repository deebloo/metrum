use super::Weight;
use std::ops::Div;

impl Div<f64> for Weight {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            nanograms: self.nanograms / rhs,
        }
    }
}

impl Div for Weight {
    type Output = f64;

    fn div(self, rhs: Self) -> f64 {
        self.nanograms / rhs.nanograms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_same_unit() {
        let w1 = Weight::from_kg(6.0);
        let w2 = Weight::from_kg(2.0);
        let quotient = w1 / w2;
        assert_eq!(quotient, 3.0);
    }

    #[test]
    fn test_div_different_units() {
        let w1 = Weight::from_kg(1.0);
        let w2 = Weight::from_g(500.0);
        let quotient = w1 / w2;
        assert_eq!(quotient, 2.0);
    }

    #[test]
    fn test_div_by_scalar() {
        let w = Weight::from_kg(6.0);
        let quotient = w / 2.0;
        assert_eq!(quotient.as_kg(), 3.0);
    }

    #[test]
    fn test_div_anticommutative() {
        let w1 = Weight::from_kg(1.0);
        let w2 = Weight::from_g(500.0);
        let quotient1 = w1 / w2;
        let quotient2 = 1.0 / (w2 / w1);
        assert_eq!(quotient1, quotient2);
    }
}
