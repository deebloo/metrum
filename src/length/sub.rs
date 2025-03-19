use super::Length;
use std::ops::Neg;

impl Neg for Length {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            nanometers: -self.nanometers,
        }
    }
}
