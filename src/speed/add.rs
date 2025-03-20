use super::Speed;
use std::ops::Add;

impl Add for Speed {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            meters_per_second: self.meters_per_second + other.meters_per_second,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_same_unit() {
        let s1 = Speed::from_mps(1.0);
        let s2 = Speed::from_mps(2.0);
        let sum = s1 + s2;
        assert_eq!(sum.as_mps(), 3.0);
    }

    #[test]
    fn test_add_different_units() {
        let s1 = Speed::from_mps(1.0);
        let s2 = Speed::from_kmph(3.6);
        let sum = s1 + s2;
        assert_eq!(sum.as_mps(), 2.0);
    }

    #[test]
    fn test_add_commutative() {
        let s1 = Speed::from_mps(1.0);
        let s2 = Speed::from_kmph(3.6);
        let sum1 = s1 + s2;
        let sum2 = s2 + s1;
        assert_eq!(sum1, sum2);
    }

    #[test]
    fn test_add_associative() {
        let s1 = Speed::from_mps(1.0);
        let s2 = Speed::from_kmph(3.6);
        let s3 = Speed::from_mph(2.23694);
        let sum1 = (s1 + s2) + s3;
        let sum2 = s1 + (s2 + s3);
        assert_eq!(sum1, sum2);
    }
}
