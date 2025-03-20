use super::Weight;
use std::ops::Mul;

impl Mul<f64> for Weight {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            nanograms: self.nanograms * rhs,
        }
    }
}

impl Mul<Weight> for f64 {
    type Output = Weight;

    fn mul(self, rhs: Weight) -> Weight {
        Weight {
            nanograms: self * rhs.nanograms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_same_unit() {
        let w = Weight::from_kg(2.0);
        let product = w * 3.0;
        assert_eq!(product.as_kg(), 6.0);
    }

    #[test]
    fn test_mul_different_units() {
        let w = Weight::from_g(1000.0);
        let product = w * 2.0;
        assert_eq!(product.as_kg(), 2.0);
    }

    #[test]
    fn test_mul_commutative() {
        let w = Weight::from_kg(2.0);
        let product1 = w * 3.0;
        let product2 = 3.0 * w;
        assert_eq!(product1, product2);
    }

    #[test]
    fn test_mul_associative() {
        let w = Weight::from_kg(2.0);
        let product1 = (w * 3.0) * 4.0;
        let product2 = w * (3.0 * 4.0);
        assert_eq!(product1, product2);
    }
}
