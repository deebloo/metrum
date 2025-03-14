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

    // Mile Subtraction Tests
    #[test]
    fn test_mile_subtraction_with_mile() {
        let val = Length::Mi(3.0);
        let rhs = Length::Mi(2.0);
        let result = val - rhs;
        if let Length::Mi(result_val) = result {
            assert_eq!(result_val, 1.0);
        } else {
            panic!("Expected result to be in miles");
        }
    }

    #[test]
    fn test_mile_subtraction_with_meter() {
        let val = Length::Mi(3.0);
        let rhs = Length::M(1609.34);
        let result = val - rhs;
        if let Length::Mi(result_val) = result {
            assert_eq!(result_val.round(), 2.0);
        } else {
            panic!("Expected result to be in miles");
        }
    }

    #[test]
    fn test_mile_subtraction_with_kilometer() {
        let val = Length::Mi(3.0);
        let rhs = Length::Km(1.60934);
        let result = val - rhs;
        if let Length::Mi(result_val) = result {
            assert_eq!(result_val.round(), 2.0);
        } else {
            panic!("Expected result to be in miles");
        }
    }

    #[test]
    fn test_mile_subtraction_with_centimeter() {
        let val = Length::Mi(3.0);
        let rhs = Length::Cm(160934.0);
        let result = val - rhs;
        if let Length::Mi(result_val) = result {
            assert_eq!(result_val.round(), 2.0);
        } else {
            panic!("Expected result to be in miles");
        }
    }

    #[test]
    fn test_mile_subtraction_with_millimeter() {
        let val = Length::Mi(3.0);
        let rhs = Length::Mm(1609000.0);
        let result = val - rhs;
        if let Length::Mi(result_val) = result {
            assert_eq!(result_val.round(), 2.0);
        } else {
            panic!("Expected result to be in miles");
        }
    }

    #[test]
    fn test_mile_subtraction_with_micrometer() {
        let val = Length::Mi(3.0);
        let rhs = Length::Um(1609000000.0);
        let result = val - rhs;
        if let Length::Mi(result_val) = result {
            assert_eq!(result_val.round(), 2.0);
        } else {
            panic!("Expected result to be in miles");
        }
    }

    #[test]
    fn test_mile_subtraction_with_nanometer() {
        let val = Length::Mi(3.0);
        let rhs = Length::Nm(1609000000000.0);
        let result = val - rhs;
        if let Length::Mi(result_val) = result {
            assert_eq!(result_val.round(), 2.0);
        } else {
            panic!("Expected result to be in miles");
        }
    }

    // Meter Subtraction Tests
    #[test]
    fn test_meter_subtraction_with_meter() {
        let val = Length::M(5.0);
        let rhs = Length::M(3.0);
        let result = val - rhs;
        if let Length::M(result_val) = result {
            assert_eq!(result_val, 2.0);
        } else {
            panic!("Expected result to be in meters");
        }
    }

    #[test]
    fn test_meter_subtraction_with_kilometer() {
        let val = Length::M(2000.0);
        let rhs = Length::Km(1.0);
        let result = val - rhs;
        if let Length::M(result_val) = result {
            assert_eq!(result_val, 1000.0);
        } else {
            panic!("Expected result to be in meters");
        }
    }

    #[test]
    fn test_meter_subtraction_with_centimeter() {
        let val = Length::M(2.0);
        let rhs = Length::Cm(100.0);
        let result = val - rhs;
        if let Length::M(result_val) = result {
            assert_eq!(result_val, 1.0);
        } else {
            panic!("Expected result to be in meters");
        }
    }

    #[test]
    fn test_meter_subtraction_with_millimeter() {
        let val = Length::M(2.0);
        let rhs = Length::Mm(1000.0);
        let result = val - rhs;
        if let Length::M(result_val) = result {
            assert_eq!(result_val, 1.0);
        } else {
            panic!("Expected result to be in meters");
        }
    }

    #[test]
    fn test_meter_subtraction_with_micrometer() {
        let val = Length::M(2.0);
        let rhs = Length::Um(1000000.0);
        let result = val - rhs;
        if let Length::M(result_val) = result {
            assert_eq!(result_val, 1.0);
        } else {
            panic!("Expected result to be in meters");
        }
    }

    #[test]
    fn test_meter_subtraction_with_nanometer() {
        let val = Length::M(2.0);
        let rhs = Length::Nm(1000000000.0);
        let result = val - rhs;
        if let Length::M(result_val) = result {
            assert_eq!(result_val, 1.0);
        } else {
            panic!("Expected result to be in meters");
        }
    }

    // Kilometer Subtraction Tests
    #[test]
    fn test_kilometer_subtraction_with_kilometer() {
        let val = Length::Km(5.0);
        let rhs = Length::Km(3.0);
        let result = val - rhs;
        if let Length::Km(result_val) = result {
            assert_eq!(result_val, 2.0);
        } else {
            panic!("Expected result to be in kilometers");
        }
    }

    #[test]
    fn test_kilometer_subtraction_with_meter() {
        let val = Length::Km(2.0);
        let rhs = Length::M(1000.0);
        let result = val - rhs;
        if let Length::Km(result_val) = result {
            assert_eq!(result_val, 1.0);
        } else {
            panic!("Expected result to be in kilometers");
        }
    }

    #[test]
    fn test_kilometer_subtraction_with_centimeter() {
        let val = Length::Km(1.0);
        let rhs = Length::Cm(100000.0);
        let result = val - rhs;
        if let Length::Km(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in kilometers");
        }
    }

    #[test]
    fn test_kilometer_subtraction_with_millimeter() {
        let val = Length::Km(1.0);
        let rhs = Length::Mm(1000000.0);
        let result = val - rhs;
        if let Length::Km(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in kilometers");
        }
    }

    #[test]
    fn test_kilometer_subtraction_with_micrometer() {
        let val = Length::Km(1.0);
        let rhs = Length::Um(1000000000.0);
        let result = val - rhs;
        if let Length::Km(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in kilometers");
        }
    }

    #[test]
    fn test_kilometer_subtraction_with_nanometer() {
        let val = Length::Km(1.0);
        let rhs = Length::Nm(1000000000000.0);
        let result = val - rhs;
        if let Length::Km(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in kilometers");
        }
    }

    // Centimeter Subtraction Tests
    #[test]
    fn test_centimeter_subtraction_with_centimeter() {
        let val = Length::Cm(5.0);
        let rhs = Length::Cm(3.0);
        let result = val - rhs;
        if let Length::Cm(result_val) = result {
            assert_eq!(result_val, 2.0);
        } else {
            panic!("Expected result to be in centimeters");
        }
    }

    #[test]
    fn test_centimeter_subtraction_with_meter() {
        let val = Length::Cm(100.0);
        let rhs = Length::M(1.0);
        let result = val - rhs;
        if let Length::Cm(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in centimeters");
        }
    }

    #[test]
    fn test_centimeter_subtraction_with_kilometer() {
        let val = Length::Cm(100000.0);
        let rhs = Length::Km(1.0);
        let result = val - rhs;
        if let Length::Cm(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in centimeters");
        }
    }

    // Millimeter Subtraction Tests
    #[test]
    fn test_millimeter_subtraction_with_millimeter() {
        let val = Length::Mm(10.0);
        let rhs = Length::Mm(5.0);
        let result = val - rhs;
        if let Length::Mm(result_val) = result {
            assert_eq!(result_val, 5.0);
        } else {
            panic!("Expected result to be in millimeters");
        }
    }

    #[test]
    fn test_millimeter_subtraction_with_meter() {
        let val = Length::Mm(1000.0);
        let rhs = Length::M(1.0);
        let result = val - rhs;
        if let Length::Mm(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in millimeters");
        }
    }

    #[test]
    fn test_millimeter_subtraction_with_kilometer() {
        let val = Length::Mm(1000000.0);
        let rhs = Length::Km(1.0);
        let result = val - rhs;
        if let Length::Mm(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in millimeters");
        }
    }

    // Micrometer Subtraction Tests
    #[test]
    fn test_micrometer_subtraction_with_micrometer() {
        let val = Length::Um(5000.0);
        let rhs = Length::Um(2000.0);
        let result = val - rhs;
        if let Length::Um(result_val) = result {
            assert_eq!(result_val, 3000.0);
        } else {
            panic!("Expected result to be in micrometers");
        }
    }

    #[test]
    fn test_micrometer_subtraction_with_meter() {
        let val = Length::Um(1000000.0);
        let rhs = Length::M(1.0);
        let result = val - rhs;
        if let Length::Um(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in micrometers");
        }
    }

    #[test]
    fn test_micrometer_subtraction_with_kilometer() {
        let val = Length::Um(1000000000.0);
        let rhs = Length::Km(1.0);
        let result = val - rhs;
        if let Length::Um(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in micrometers");
        }
    }

    // Nanometer Subtraction Tests
    #[test]
    fn test_nanometer_subtraction_with_nanometer() {
        let val = Length::Nm(5000.0);
        let rhs = Length::Nm(3000.0);
        let result = val - rhs;
        if let Length::Nm(result_val) = result {
            assert_eq!(result_val, 2000.0);
        } else {
            panic!("Expected result to be in nanometers");
        }
    }

    #[test]
    fn test_nanometer_subtraction_with_meter() {
        let val = Length::Nm(1000000000.0);
        let rhs = Length::M(1.0);
        let result = val - rhs;
        if let Length::Nm(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in nanometers");
        }
    }

    #[test]
    fn test_nanometer_subtraction_with_kilometer() {
        let val = Length::Nm(1000000000000.0);
        let rhs = Length::Km(1.0);
        let result = val - rhs;
        if let Length::Nm(result_val) = result {
            assert_eq!(result_val, 0.0);
        } else {
            panic!("Expected result to be in nanometers");
        }
    }
}
