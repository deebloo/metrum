use crate::temp::Temp;
use std::ops::Sub;

impl Sub for Temp {
    type Output = Temp;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_k(self.as_k() - rhs.as_k())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn should_sub_from_c() {
    //     let cc = Temp::from_c(10.) - Temp::from_c(10.);
    //     let cf = Temp::from_c(10.) - Temp::from_f(50.);
    //     let ck = Temp::from_c(10.) - Temp::from_k(283.15);

    //     assert_eq!(cc, Temp::from_c(0.));
    //     assert_eq!(cf, Temp::from_c(0.));
    //     assert_eq!(ck, Temp::from_c(0.));
    // }

    // #[test]
    // fn should_sub_from_f() {
    //     let ff = Temp::from_f(100.) - Temp::from_f(100.);
    //     let fc = Temp::from_f(100.) - Temp::from_c(37.778);
    //     let fk = Temp::from_f(100.) - Temp::from_k(310.928);

    //     assert_eq!(ff.round(), Temp::from_f(0.));
    //     assert_eq!(fc.round(), Temp::from_f(0.));
    //     assert_eq!(fk.round(), Temp::from_f(0.));
    // }

    // #[test]
    // fn should_sub_from_k() {
    //     let kk = Temp::from_k(300.) - Temp::from_k(300.);
    //     let kc = Temp::from_k(300.) - Temp::from_c(26.85);
    //     let kf = Temp::from_k(300.) - Temp::from_f(80.33);

    //     assert_eq!(kk, Temp::from_k(0.));
    //     assert_eq!(kf, Temp::from_k(0.));
    //     assert_eq!(kc, Temp::from_k(0.));
    // }
}
