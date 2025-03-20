use super::Weight;
use std::ops::{Neg, Sub};

impl Sub for Weight {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            nanograms: self.nanograms - other.nanograms,
        }
    }
}

impl Neg for Weight {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            nanograms: -self.nanograms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_same_unit() {
        let w1 = Weight::from_kg(2.0);
        let w2 = Weight::from_kg(1.0);
        let diff = w1 - w2;
        assert_eq!(diff.as_kg(), 1.0);
    }

    #[test]
    fn test_sub_different_units() {
        let w1 = Weight::from_kg(2.0);
        let w2 = Weight::from_g(500.0);
        let diff = w1 - w2;
        assert_eq!(diff.as_kg(), 1.5);
    }

    #[test]
    fn test_sub_anticommutative() {
        let w1 = Weight::from_kg(1.0);
        let w2 = Weight::from_g(500.0);
        let diff1 = w1 - w2;
        let diff2 = -(w2 - w1);
        assert_eq!(diff1, diff2);
    }

    #[test]
    fn test_neg() {
        let w = Weight::from_kg(1.0);
        let neg = -w;
        assert_eq!(neg.as_kg(), -1.0);
    }
}
