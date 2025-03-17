use crate::temp::Temp;
use std::ops::Mul;

impl Mul for Temp {
    type Output = Temp;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_k(self.as_k() * rhs.as_k())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn should_mul_to_c() {
//         let cc = Temp::from_c(10.) * Temp::from_c(2.);
//         let cf = Temp::from_c(10.) * Temp::from_f(35.6);
//         let ck = Temp::from_c(10.) * Temp::from_k(275.15);

//         assert_eq!(cc, Temp::from_c(20.));
//         assert_eq!(cf, Temp::from_c(20.));
//         assert_eq!(ck, Temp::from_c(20.));
//     }

//     #[test]
//     fn should_mul_to_f() {
//         let ff = Temp::from_f(2.) * Temp::from_f(32.);
//         let fc = Temp::from_f(2.) * Temp::from_c(0.);
//         let fk = Temp::from_f(2.) * Temp::from_k(273.15);

//         assert_eq!(ff, Temp::from_f(64.));
//         assert_eq!(fc, Temp::from_f(64.));
//         assert_eq!(fk, Temp::from_f(64.));
//     }

//     #[test]
//     fn should_mul_to_k() {
//         let kk = Temp::from_k(15.) * Temp::from_k(273.15);
//         let kc = Temp::from_k(15.) * Temp::from_c(0.);
//         let kf = Temp::from_k(15.) * Temp::from_f(32.);

//         assert_eq!(kk, Temp::from_k(4097.25));
//         assert_eq!(kf, Temp::from_k(4097.25));
//         assert_eq!(kc, Temp::from_k(4097.25));
//     }
// }
