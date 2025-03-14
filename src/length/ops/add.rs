use crate::length::Length;
use std::ops::Add;

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> Self::Output {
        match self {
            Length::In(val) => {
                let target: f64 = rhs.as_in().into();
                Length::In(val + target)
            }

            Length::Ft(val) => {
                let target: f64 = rhs.as_ft().into();
                Length::Ft(val + target)
            }

            Length::Yd(val) => {
                let target: f64 = rhs.as_yd().into();
                Length::Yd(val + target)
            }

            Length::Mi(val) => {
                let target: f64 = rhs.as_mi().into();
                Length::Mi(val + target)
            }

            Length::M(val) => {
                let target: f64 = rhs.as_m().into();
                Length::M(val + target)
            }

            Length::Km(val) => {
                let target: f64 = rhs.as_km().into();
                Length::Km(val + target)
            }

            Length::Cm(val) => {
                let target: f64 = rhs.as_cm().into();
                Length::Cm(val + target)
            }

            Length::Mm(val) => {
                let target: f64 = rhs.as_mm().into();
                Length::Mm(val + target)
            }

            Length::Um(val) => {
                let target: f64 = rhs.as_um().into();
                Length::Um(val + target)
            }

            Length::Nm(val) => {
                let target: f64 = rhs.as_nm().into();
                Length::Nm(val + target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inch_addition_with_inch() {
        let val = Length::In(5.0);
        let rhs = Length::In(3.0);
        let result = val + rhs;
        if let Length::In(result_val) = result {
            assert_eq!(result_val, 8.0);
        } else {
            panic!("Expected result to be in inches");
        }
    }

    #[test]
    fn test_inch_addition_with_foot() {
        let val = Length::In(12.0); // 12 inches
        let rhs = Length::Ft(1.0); // 1 foot (12 inches)
        let result = val + rhs;
        if let Length::In(result_val) = result {
            assert_eq!(result_val, 24.0); // 24 inches
        } else {
            panic!("Expected result to be in inches");
        }
    }

    #[test]
    fn test_inch_addition_with_yard() {
        let val = Length::In(36.0); // 36 inches
        let rhs = Length::Yd(1.0); // 1 yard (36 inches)
        let result = val + rhs;
        if let Length::In(result_val) = result {
            assert_eq!(result_val, 72.0); // 72 inches
        } else {
            panic!("Expected result to be in inches");
        }
    }

    #[test]
    fn test_inch_addition_with_mile() {
        let val = Length::In(63360.0); // 5280 inches (1 mile)
        let rhs = Length::Mi(1.0); // 1 mile (5280 inches)
        let result = val + rhs;
        if let Length::In(result_val) = result {
            assert_eq!(result_val, 126720.0); // 10560 inches (2 miles)
        } else {
            panic!("Expected result to be in inches");
        }
    }

    #[test]
    fn test_inch_addition_with_meter() {
        let val = Length::In(39.37); // 1 meter (in inches)
        let rhs = Length::M(1.0); // 1 meter
        let result = val + rhs;
        if let Length::In(result_val) = result {
            assert_eq!(result_val, 78.7401); // 2 meters in inches
        } else {
            panic!("Expected result to be in inches");
        }
    }

    #[test]
    fn test_inch_addition_with_kilometer() {
        let val = Length::In(39370.1); // 1 kilometer (in inches)
        let rhs = Length::Km(1.0); // 1 kilometer
        let result = val + rhs;
        if let Length::In(result_val) = result {
            assert_eq!(result_val, 78740.2); // 2 kilometers in inches
        } else {
            panic!("Expected result to be in inches");
        }
    }

    #[test]
    fn test_foot_addition_with_foot() {
        let val = Length::Ft(3.0);
        let rhs = Length::Ft(2.0);
        let result = val + rhs;
        if let Length::Ft(result_val) = result {
            assert_eq!(result_val, 5.0);
        } else {
            panic!("Expected result to be in feet");
        }
    }

    #[test]
    fn test_foot_addition_with_yard() {
        let val = Length::Ft(3.0); // 3 feet
        let rhs = Length::Yd(1.0); // 1 yard (3 feet)
        let result = val + rhs;
        if let Length::Ft(result_val) = result {
            assert_eq!(result_val, 6.0); // 6 feet
        } else {
            panic!("Expected result to be in feet");
        }
    }

    #[test]
    fn test_foot_addition_with_mile() {
        let val = Length::Ft(5280.0); // 5280 feet (1 mile)
        let rhs = Length::Mi(1.0); // 1 mile (5280 feet)
        let result = val + rhs;
        if let Length::Ft(result_val) = result {
            assert_eq!(result_val, 10560.0); // 2 miles (10560 feet)
        } else {
            panic!("Expected result to be in feet");
        }
    }

    #[test]
    fn test_foot_addition_with_meter() {
        let val = Length::Ft(3.28084); // 1 meter in feet
        let rhs = Length::M(1.0); // 1 meter
        let result = val + rhs;
        if let Length::Ft(result_val) = result {
            assert_eq!(result_val, 6.56168); // 2 meters in feet
        } else {
            panic!("Expected result to be in feet");
        }
    }

    #[test]
    fn test_foot_addition_with_kilometer() {
        let val = Length::Ft(3280.84); // 1 kilometer in feet
        let rhs = Length::Km(1.0); // 1 kilometer
        let result = val + rhs;
        if let Length::Ft(result_val) = result {
            assert_eq!(result_val, 6561.68); // 2 kilometers in feet
        } else {
            panic!("Expected result to be in feet");
        }
    }

    #[test]
    fn test_yard_addition_with_yard() {
        let val = Length::Yd(3.0);
        let rhs = Length::Yd(2.0);
        let result = val + rhs;
        if let Length::Yd(result_val) = result {
            assert_eq!(result_val, 5.0);
        } else {
            panic!("Expected result to be in yards");
        }
    }

    #[test]
    fn test_yard_addition_with_mile() {
        let val = Length::Yd(1760.0); // 1760 yards (1 mile)
        let rhs = Length::Mi(1.0); // 1 mile (1760 yards)
        let result = val + rhs;
        if let Length::Yd(result_val) = result {
            assert_eq!(result_val, 3520.0); // 2 miles (3520 yards)
        } else {
            panic!("Expected result to be in yards");
        }
    }

    #[test]
    fn test_yard_addition_with_meter() {
        let val = Length::Yd(1.09361); // 1 meter (in yards)
        let rhs = Length::M(1.0); // 1 meter
        let result = val + rhs;
        if let Length::Yd(result_val) = result {
            assert_eq!(result_val, 2.18722); // 2 meters in yards
        } else {
            panic!("Expected result to be in yards");
        }
    }

    #[test]
    fn test_yard_addition_with_kilometer() {
        let val = Length::Yd(1093.61); // 1 kilometer in yards
        let rhs = Length::Km(1.0); // 1 kilometer
        let result = val + rhs;
        if let Length::Yd(result_val) = result {
            assert_eq!(result_val, 2187.22); // 2 kilometers in yards
        } else {
            panic!("Expected result to be in yards");
        }
    }

    #[test]
    fn test_mile_addition_with_mile() {
        let val = Length::Mi(1.0); // 1 mile
        let rhs = Length::Mi(2.0); // 2 miles
        let result = val + rhs;
        if let Length::Mi(result_val) = result {
            assert_eq!(result_val, 3.0); // 3 miles
        } else {
            panic!("Expected result to be in miles");
        }
    }

    #[test]
    fn test_mile_addition_with_meter() {
        let val = Length::Mi(1.0); // 1 mile
        let rhs = Length::M(1609.34); // 1609.34 meters (1 mile)
        let result = val + rhs;
        if let Length::Mi(result_val) = result {
            assert_eq!(result_val.round(), 2.0); // 2 miles
        } else {
            panic!("Expected result to be in miles");
        }
    }

    #[test]
    fn test_mile_addition_with_kilometer() {
        let val = Length::Mi(1.0); // 1 mile
        let rhs = Length::Km(1.60934); // 1.60934 kilometers (1 mile)
        let result = val + rhs;
        if let Length::Mi(result_val) = result {
            assert_eq!(result_val.round(), 2.0); // 2 miles
        } else {
            panic!("Expected result to be in miles");
        }
    }

    #[test]
    fn test_meter_addition_with_meter() {
        let val = Length::M(5.0); // 5 meters
        let rhs = Length::M(3.0); // 3 meters
        let result = val + rhs;
        if let Length::M(result_val) = result {
            assert_eq!(result_val, 8.0); // 8 meters
        } else {
            panic!("Expected result to be in meters");
        }
    }

    #[test]
    fn test_meter_addition_with_kilometer() {
        let val = Length::M(1000.0); // 1000 meters (1 kilometer)
        let rhs = Length::Km(1.0); // 1 kilometer
        let result = val + rhs;
        if let Length::M(result_val) = result {
            assert_eq!(result_val, 2000.0); // 2 kilometers in meters
        } else {
            panic!("Expected result to be in meters");
        }
    }

    #[test]
    fn test_centimeter_addition_with_centimeter() {
        let val = Length::Cm(5.0); // 5 centimeters
        let rhs = Length::Cm(3.0); // 3 centimeters
        let result = val + rhs;
        if let Length::Cm(result_val) = result {
            assert_eq!(result_val, 8.0); // 8 centimeters
        } else {
            panic!("Expected result to be in centimeters");
        }
    }

    #[test]
    fn test_centimeter_addition_with_meter() {
        let val = Length::Cm(100.0); // 100 centimeters (1 meter)
        let rhs = Length::M(1.0); // 1 meter
        let result = val + rhs;
        if let Length::Cm(result_val) = result {
            assert_eq!(result_val, 200.0); // 2 meters in centimeters
        } else {
            panic!("Expected result to be in centimeters");
        }
    }

    #[test]
    fn test_centimeter_addition_with_kilometer() {
        let val = Length::Cm(100000.0); // 100000 centimeters (1 kilometer)
        let rhs = Length::Km(1.0); // 1 kilometer
        let result = val + rhs;
        if let Length::Cm(result_val) = result {
            assert_eq!(result_val, 200000.0); // 2 kilometers in centimeters
        } else {
            panic!("Expected result to be in centimeters");
        }
    }

    #[test]
    fn test_millimeter_addition_with_millimeter() {
        let val = Length::Mm(10.0); // 10 millimeters
        let rhs = Length::Mm(5.0); // 5 millimeters
        let result = val + rhs;
        if let Length::Mm(result_val) = result {
            assert_eq!(result_val, 15.0); // 15 millimeters
        } else {
            panic!("Expected result to be in millimeters");
        }
    }

    #[test]
    fn test_millimeter_addition_with_meter() {
        let val = Length::Mm(1000.0); // 1000 millimeters (1 meter)
        let rhs = Length::M(1.0); // 1 meter
        let result = val + rhs;
        if let Length::Mm(result_val) = result {
            assert_eq!(result_val, 2000.0); // 2 meters in millimeters
        } else {
            panic!("Expected result to be in millimeters");
        }
    }

    #[test]
    fn test_millimeter_addition_with_kilometer() {
        let val = Length::Mm(1000000.0); // 1000000 millimeters (1 kilometer)
        let rhs = Length::Km(1.0); // 1 kilometer
        let result = val + rhs;
        if let Length::Mm(result_val) = result {
            assert_eq!(result_val, 2000000.0); // 2 kilometers in millimeters
        } else {
            panic!("Expected result to be in millimeters");
        }
    }

    #[test]
    fn test_micrometer_addition_with_micrometer() {
        let val = Length::Um(5000.0); // 5000 micrometers
        let rhs = Length::Um(2000.0); // 2000 micrometers
        let result = val + rhs;
        if let Length::Um(result_val) = result {
            assert_eq!(result_val, 7000.0); // 7000 micrometers
        } else {
            panic!("Expected result to be in micrometers");
        }
    }

    #[test]
    fn test_micrometer_addition_with_meter() {
        let val = Length::Um(1000000.0); // 1000000 micrometers (1 meter)
        let rhs = Length::M(1.0); // 1 meter
        let result = val + rhs;
        if let Length::Um(result_val) = result {
            assert_eq!(result_val, 2000000.0); // 2 meters in micrometers
        } else {
            panic!("Expected result to be in micrometers");
        }
    }

    #[test]
    fn test_micrometer_addition_with_kilometer() {
        let val = Length::Um(1000000000.0); // 1000000000 micrometers (1 kilometer)
        let rhs = Length::Km(1.0); // 1 kilometer
        let result = val + rhs;
        if let Length::Um(result_val) = result {
            assert_eq!(result_val, 2000000000.0); // 2 kilometers in micrometers
        } else {
            panic!("Expected result to be in micrometers");
        }
    }

    #[test]
    fn test_nanometer_addition_with_nanometer() {
        let val = Length::Nm(5000.0); // 5000 nanometers
        let rhs = Length::Nm(3000.0); // 3000 nanometers
        let result = val + rhs;
        if let Length::Nm(result_val) = result {
            assert_eq!(result_val, 8000.0); // 8000 nanometers
        } else {
            panic!("Expected result to be in nanometers");
        }
    }

    #[test]
    fn test_nanometer_addition_with_meter() {
        let val = Length::Nm(1000000000.0); // 1000000000 nanometers (1 meter)
        let rhs = Length::M(1.0); // 1 meter
        let result = val + rhs;
        if let Length::Nm(result_val) = result {
            assert_eq!(result_val, 2000000000.0); // 2 meters in nanometers
        } else {
            panic!("Expected result to be in nanometers");
        }
    }

    #[test]
    fn test_nanometer_addition_with_kilometer() {
        let val = Length::Nm(1000000000000.0); // 1000000000000 nanometers (1 kilometer)
        let rhs = Length::Km(1.0); // 1 kilometer
        let result = val + rhs;
        if let Length::Nm(result_val) = result {
            assert_eq!(result_val, 2000000000000.0); // 2 kilometers in nanometers
        } else {
            panic!("Expected result to be in nanometers");
        }
    }
}
