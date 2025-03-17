use crate::length::Length;
use std::ops::Add;

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::In(val) => {
                let target: f64 = rhs.as_in().into();
                Self::In(val + target)
            }

            Self::Ft(val) => {
                let target: f64 = rhs.as_ft().into();
                Self::Ft(val + target)
            }

            Self::Yd(val) => {
                let target: f64 = rhs.as_yd().into();
                Self::Yd(val + target)
            }

            Self::Mi(val) => {
                let target: f64 = rhs.as_mi().into();
                Self::Mi(val + target)
            }

            Self::M(val) => {
                let target: f64 = rhs.as_m().into();
                Self::M(val + target)
            }

            Self::Km(val) => {
                let target: f64 = rhs.as_km().into();
                Self::Km(val + target)
            }

            Self::Cm(val) => {
                let target: f64 = rhs.as_cm().into();
                Self::Cm(val + target)
            }

            Self::Mm(val) => {
                let target: f64 = rhs.as_mm().into();
                Self::Mm(val + target)
            }

            Self::Um(val) => {
                let target: f64 = rhs.as_um().into();
                Self::Um(val + target)
            }

            Self::Nm(val) => {
                let target: f64 = rhs.as_nm().into();
                Self::Nm(val + target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_combinations(base_unit: Length) {
        let units = vec![
            Length::In(-63360.),
            Length::Ft(-5280.),
            Length::Yd(-1760.),
            Length::Mi(-1.),
            Length::Km(-1.609344),
            Length::Cm(-160934.4),
            Length::Mm(-1.609344e+6),
            Length::Um(-1.609344e+9),
            Length::Nm(-1.609344e+12),
        ];

        for unit in units {
            let res: f64 = (base_unit + unit).round_to(10.).into();
            // Results should All equal 0
            assert_eq!(res, 0.);
        }
    }

    #[test]
    fn inch_addition() {
        test_combinations(Length::In(63360.));
    }

    #[test]
    fn foot_addition() {
        test_combinations(Length::Ft(5280.));
    }

    #[test]
    fn yard_addition() {
        test_combinations(Length::Yd(1760.));
    }

    #[test]
    fn mile_addition() {
        test_combinations(Length::Mi(1.));
    }

    #[test]
    fn km_addition() {
        test_combinations(Length::Km(1.609344));
    }

    #[test]
    fn cm_addition() {
        test_combinations(Length::Cm(160934.4));
    }

    #[test]
    fn mm_addition() {
        test_combinations(Length::Mm(1.609344e+6));
    }

    #[test]
    fn um_addition() {
        test_combinations(Length::Um(1.609344e+9));
    }

    #[test]
    fn nm_addition() {
        test_combinations(Length::Nm(1.609344e+12));
    }
}
