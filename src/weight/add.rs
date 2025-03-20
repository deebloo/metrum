use super::Weight;
use std::ops::Add;

impl Add for Weight {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            nanograms: self.nanograms + other.nanograms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_same_unit() {
        let w1 = Weight::from_kg(1.0);
        let w2 = Weight::from_kg(2.0);
        let sum = w1 + w2;
        assert_eq!(sum.as_kg(), 3.0);
    }

    #[test]
    fn test_add_different_units() {
        let w1 = Weight::from_kg(1.0);
        let w2 = Weight::from_g(500.0);
        let sum = w1 + w2;
        assert_eq!(sum.as_kg(), 1.5);
    }

    #[test]
    fn test_add_commutative() {
        let w1 = Weight::from_kg(1.0);
        let w2 = Weight::from_g(1000.0);
        let sum1 = w1 + w2;
        let sum2 = w2 + w1;
        assert_eq!(sum1, sum2);
    }

    #[test]
    fn test_add_associative() {
        let w1 = Weight::from_kg(1.0);
        let w2 = Weight::from_g(500.0);
        let w3 = Weight::from_mg(500_000.0);
        let sum1 = (w1 + w2) + w3;
        let sum2 = w1 + (w2 + w3);
        assert_eq!(sum1, sum2);
    }
}
