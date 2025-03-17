use crate::temp::Temp;
use std::ops::Sub;

impl Sub for Temp {
    type Output = Temp;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_k(self.kelvin - rhs.kelvin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_subtract_to_c() {
        let value: Temp = Temp::from_c(0.) - Temp::from_c(0.);

        assert_eq!(value.as_c(), 0.);
    }
}
