use super::Speed;
use std::cmp::PartialEq;

impl PartialEq for Speed {
    fn eq(&self, other: &Self) -> bool {
        let mut epsilon: f64 = f64::EPSILON;

        if (self.meters_per_second - other.meters_per_second).abs() <= epsilon {
            return true;
        }

        epsilon = if self.meters_per_second > other.meters_per_second {
            epsilon * self.meters_per_second.abs()
        } else {
            epsilon * other.meters_per_second.abs()
        };

        (self.meters_per_second - other.meters_per_second).abs() <= epsilon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_same_unit() {
        let s1 = Speed::from_mps(1.0);
        let s2 = Speed::from_mps(1.0);
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_eq_different_units() {
        let s1 = Speed::from_mps(1.0);
        let s2 = Speed::from_kmph(3.6);
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_eq_outside_epsilon() {
        let s1 = Speed::from_mps(1.0);
        let s2 = Speed::from_mps(1.0 + f64::EPSILON); // Just outside epsilon
        assert_ne!(s1, s2);
    }
}
