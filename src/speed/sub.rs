use super::Speed;
use std::ops::{Neg, Sub};

impl Sub for Speed {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            meters_per_second: self.meters_per_second - other.meters_per_second,
        }
    }
}

impl Neg for Speed {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            meters_per_second: -self.meters_per_second,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_same_unit() {
        let s1 = Speed::from_mps(2.0);
        let s2 = Speed::from_mps(1.0);
        let diff = s1 - s2;
        assert_eq!(diff.as_mps(), 1.0);
    }

    #[test]
    fn test_sub_different_units() {
        let s1 = Speed::from_mps(2.0);
        let s2 = Speed::from_kmph(3.6);
        let diff = s1 - s2;
        assert_eq!(diff.as_mps(), 1.0);
    }

    #[test]
    fn test_sub_anticommutative() {
        let s1 = Speed::from_mps(1.0);
        let s2 = Speed::from_kmph(3.6);
        let diff1 = s1 - s2;
        let diff2 = -(s2 - s1);
        assert_eq!(diff1, diff2);
    }

    #[test]
    fn test_neg() {
        let s = Speed::from_mps(1.0);
        let neg = -s;
        assert_eq!(neg.as_mps(), -1.0);
    }
}
