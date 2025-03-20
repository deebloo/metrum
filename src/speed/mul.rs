use super::Speed;
use std::ops::Mul;

impl Mul<f64> for Speed {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            meters_per_second: self.meters_per_second * rhs,
        }
    }
}

impl Mul<Speed> for f64 {
    type Output = Speed;

    fn mul(self, rhs: Speed) -> Speed {
        Speed {
            meters_per_second: self * rhs.meters_per_second,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_same_unit() {
        let s = Speed::from_mps(2.0);
        let product = s * 3.0;
        assert_eq!(product.as_mps(), 6.0);
    }

    #[test]
    fn test_mul_different_units() {
        let s = Speed::from_kmph(36.0);
        let product = s * 2.0;
        assert_eq!(product.as_mps(), 20.0);
    }

    #[test]
    fn test_mul_commutative() {
        let s = Speed::from_mps(2.0);
        let product1 = s * 3.0;
        let product2 = 3.0 * s;
        assert_eq!(product1, product2);
    }

    #[test]
    fn test_mul_associative() {
        let s = Speed::from_mps(2.0);
        let product1 = (s * 3.0) * 4.0;
        let product2 = s * (3.0 * 4.0);
        assert_eq!(product1, product2);
    }
}
