use crate::temp::Temp;
use std::ops::Add;

impl Add for Temp {
    type Output = Temp;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_k(self.as_k() + rhs.as_k())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_to_c() {
        let value = Temp::from_c(0.) + Temp::from_f(32.);

        // assert_eq!(value.as_c(), 0.);
        // assert_eq!(value.as_f(), 32.);
        assert_eq!(value.as_k(), 273.15);
    }
}
