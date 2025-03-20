use super::Weight;
use std::cmp::PartialEq;

impl PartialEq for Weight {
    fn eq(&self, other: &Self) -> bool {
        let mut epsilon: f64 = f64::EPSILON;

        if (self.nanograms - other.nanograms).abs() <= epsilon {
            return true;
        }

        epsilon = if self.nanograms > other.nanograms {
            epsilon * self.nanograms.abs()
        } else {
            epsilon * other.nanograms.abs()
        };

        (self.nanograms - other.nanograms).abs() <= epsilon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_same_unit() {
        let w1 = Weight::from_kg(1.0);
        let w2 = Weight::from_kg(1.0);
        assert_eq!(w1, w2);
    }

    #[test]
    fn test_eq_different_units() {
        let w1 = Weight::from_kg(1.0);
        let w2 = Weight::from_g(1000.0);
        assert_eq!(w1, w2);
    }

    #[test]
    fn test_eq_outside_epsilon() {
        let w1 = Weight::from_kg(1.0);
        let w2 = Weight::from_kg(1.0 + f64::EPSILON); // Just outside epsilon
        assert_ne!(w1, w2);
    }
}
