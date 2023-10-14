use crate::temp::Temp;
use std::ops::Add;

impl Add<Self> for Temp {
    type Output = Temp;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::C(val) => {
                let target: f64 = rhs.as_c().into();

                Self::C(val + target)
            }
            Self::F(val) => {
                let target: f64 = rhs.as_f().into();

                Self::F(val + target)
            }
            Self::K(val) => {
                let target: f64 = rhs.as_k().into();

                Self::K(val + target)
            }
        }
    }
}

impl Add<f64> for Temp {
    type Output = Temp;

    fn add(self, rhs: f64) -> Self::Output {
        match self {
            Self::C(val) => Self::C(val + rhs),
            Self::F(val) => Self::F(val + rhs),
            Self::K(val) => Self::K(val + rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_to_c() {
        let cc = Temp::C(10.) + Temp::C(10.);
        let cf = Temp::C(10.) + Temp::F(50.);
        let ck = Temp::C(10.) + Temp::K(283.15);
        let cf64 = Temp::C(10.) + 10.;

        assert_eq!(cc, Temp::C(20.));
        assert_eq!(cf, Temp::C(20.));
        assert_eq!(ck, Temp::C(20.));
        assert_eq!(cf64, Temp::C(20.));
    }

    #[test]
    fn should_add_to_f() {
        let ff = Temp::F(100.) + Temp::F(100.);
        let fc = Temp::F(100.) + Temp::C(37.778);
        let fk = Temp::F(100.) + Temp::K(310.928);
        let ff64 = Temp::F(10.) + 10.;

        assert_eq!(ff, Temp::F(200.));
        assert_eq!(fc, Temp::F(200.0004));
        assert_eq!(fk, Temp::F(200.00040000000004));
        assert_eq!(ff64, Temp::F(20.));
    }

    #[test]
    fn should_add_to_k() {
        let kk = Temp::K(300.) + Temp::K(300.);
        let kc = Temp::K(300.) + Temp::C(26.85);
        let kf = Temp::K(300.) + Temp::F(80.33);
        let kf64 = Temp::K(10.) + 10.;

        assert_eq!(kk, Temp::K(600.));
        assert_eq!(kf, Temp::K(600.));
        assert_eq!(kc, Temp::K(600.));
        assert_eq!(kf64, Temp::K(20.));
    }
}
