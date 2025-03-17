use crate::length::Length;
use std::ops::Sub;

impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Length) -> Self::Output {
        match self {
            Length::In(val) => {
                let target: f64 = rhs.as_in().into();
                Length::In(val - target)
            }

            Length::Ft(val) => {
                let target: f64 = rhs.as_ft().into();
                Length::Ft(val - target)
            }

            Length::Yd(val) => {
                let target: f64 = rhs.as_yd().into();
                Length::Yd(val - target)
            }

            Length::Mi(val) => {
                let target: f64 = rhs.as_mi().into();
                Length::Mi(val - target)
            }

            Length::M(val) => {
                let target: f64 = rhs.as_m().into();
                Length::M(val - target)
            }

            Length::Km(val) => {
                let target: f64 = rhs.as_km().into();
                Length::Km(val - target)
            }

            Length::Cm(val) => {
                let target: f64 = rhs.as_cm().into();
                Length::Cm(val - target)
            }

            Length::Mm(val) => {
                let target: f64 = rhs.as_mm().into();
                Length::Mm(val - target)
            }

            Length::Um(val) => {
                let target: f64 = rhs.as_um().into();
                Length::Um(val - target)
            }

            Length::Nm(val) => {
                let target: f64 = rhs.as_nm().into();
                Length::Nm(val - target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_combinations(base_unit: Length) {
        let units = vec![
            Length::In(63360.),
            Length::Ft(5280.),
            Length::Yd(1760.),
            Length::Mi(1.),
            Length::Km(1.609344),
            Length::Cm(160934.4),
            Length::Mm(1.609344e+6),
            Length::Um(1.609344e+9),
            Length::Nm(1.609344e+12),
        ];

        for unit in units {
            let res: f64 = (base_unit - unit).round_to(10.).into();
            // Results should All equal 0
            assert_eq!(res, 0.);
        }
    }

    #[test]
    fn inch_subscription() {
        test_combinations(Length::In(63360.));
    }

    #[test]
    fn foot_subscription() {
        test_combinations(Length::Ft(5280.));
    }

    #[test]
    fn yard_subscription() {
        test_combinations(Length::Yd(1760.));
    }

    #[test]
    fn mile_subscription() {
        test_combinations(Length::Mi(1.));
    }

    #[test]
    fn km_subscription() {
        test_combinations(Length::Km(1.609344));
    }

    #[test]
    fn cm_subscription() {
        test_combinations(Length::Cm(160934.4));
    }

    #[test]
    fn mm_subscription() {
        test_combinations(Length::Mm(1.609344e+6));
    }

    #[test]
    fn um_subscription() {
        test_combinations(Length::Um(1.609344e+9));
    }

    #[test]
    fn nm_subscription() {
        test_combinations(Length::Nm(1.609344e+12));
    }
}
