mod ops;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Length {
    In(f64), // Inches
    Ft(f64), // Feet
    Yd(f64), // Yards
    Mi(f64), // Miles
    M(f64),  // Meters
    Km(f64), // Kilometers
    Cm(f64), // Centimeters
    Mm(f64), // Millimeters
    Um(f64), // Micrometers
    Nm(f64), // Nanometers
}

impl Length {
    pub fn as_in(&self) -> Self {
        match self {
            Self::In(_) => *self,
            Self::Ft(val) => Self::In(val * 12.),
            Self::Yd(val) => Self::In(val * 36.),
            Self::Mi(val) => Self::In(val * 63360.),
            Self::M(val) => Self::In(val * 39.3701),
            Self::Km(val) => Self::In(val * 39370.1),
            Self::Cm(val) => Self::In(val / 2.54),
            Self::Mm(val) => Self::In(val / 25.4),
            Self::Um(val) => Self::In(val / 25400.),
            Self::Nm(val) => Self::In(val / 2.54e+7),
        }
    }

    pub fn as_ft(&self) -> Self {
        match self {
            Self::In(val) => Self::Ft(val / 12.),
            Self::Ft(_) => *self,
            Self::Yd(val) => Self::Ft(val * 3.),
            Self::Mi(val) => Self::Ft(val * 5280.),
            Self::M(val) => Self::Ft(val * 3.28084),
            Self::Km(val) => Self::Ft(val * 3280.84),
            Self::Cm(val) => Self::Ft(val / 30.48),
            Self::Mm(val) => Self::Ft(val / 304.8),
            Self::Um(val) => Self::Ft(val / 304800.),
            Self::Nm(val) => Self::Ft(val / 3.048e+8),
        }
    }

    pub fn as_yd(&self) -> Self {
        match self {
            Self::In(val) => Self::Yd(val / 36.),
            Self::Ft(val) => Self::Yd(val / 3.),
            Self::Yd(_) => *self,
            Self::Mi(val) => Self::Yd(val * 1760.),
            Self::M(val) => Self::Yd(val * 1.09361),
            Self::Km(val) => Self::Yd(val * 1093.61),
            Self::Cm(val) => Self::Yd(val / 91.44),
            Self::Mm(val) => Self::Yd(val / 914.4),
            Self::Um(val) => Self::Yd(val / 914400.),
            Self::Nm(val) => Self::Yd(val / 9.144e+8),
        }
    }

    pub fn as_mi(&self) -> Self {
        match self {
            Self::In(val) => Self::Mi(val / 63360.),
            Self::Ft(val) => Self::Mi(val / 5280.),
            Self::Yd(val) => Self::Mi(val / 1760.),
            Self::Mi(_) => *self,
            Self::M(val) => Self::Mi(val * 0.000621371),
            Self::Km(val) => Self::Mi(val * 0.621371),
            Self::Cm(val) => Self::Mi(val * 0.00000621371),
            Self::Mm(val) => Self::Mi(val * 0.000000621371),
            Self::Um(val) => Self::Mi(val * 0.000000000621371),
            Self::Nm(val) => Self::Mi(val * 0.000000000000621371),
        }
    }

    pub fn as_m(&self) -> Self {
        match self {
            Self::In(val) => Self::M(val / 39.3701),
            Self::Ft(val) => Self::M(val / 3.28084),
            Self::Yd(val) => Self::M(val / 1.09361),
            Self::Mi(val) => Self::M(val * 1609.34),
            Self::M(_) => *self,
            Self::Km(val) => Self::M(val * 1000.),
            Self::Cm(val) => Self::M(val / 100.),
            Self::Mm(val) => Self::M(val / 1000.),
            Self::Um(val) => Self::M(val / 1000000.),
            Self::Nm(val) => Self::M(val / 1000000000.),
        }
    }

    pub fn as_km(&self) -> Self {
        match self {
            Self::In(val) => Self::Km(val / 39370.1),
            Self::Ft(val) => Self::Km(val / 3280.84),
            Self::Yd(val) => Self::Km(val / 1093.61),
            Self::Mi(val) => Self::Km(val * 1.60934),
            Self::M(val) => Self::Km(val / 1000.),
            Self::Km(_) => *self,
            Self::Cm(val) => Self::Km(val / 100000.),
            Self::Mm(val) => Self::Km(val / 1000000.),
            Self::Um(val) => Self::Km(val / 1000000000.),
            Self::Nm(val) => Self::Km(val / 1000000000000.),
        }
    }

    pub fn as_cm(&self) -> Self {
        match self {
            Self::In(val) => Self::Cm(val * 2.54),
            Self::Ft(val) => Self::Cm(val * 30.48),
            Self::Yd(val) => Self::Cm(val * 91.44),
            Self::Mi(val) => Self::Cm(val * 160934.4),
            Self::M(val) => Self::Cm(val * 100.),
            Self::Km(val) => Self::Cm(val * 100000.),
            Self::Cm(_) => *self,
            Self::Mm(val) => Self::Cm(val / 10.),
            Self::Um(val) => Self::Cm(val / 10000.),
            Self::Nm(val) => Self::Cm(val / 10000000.),
        }
    }

    pub fn as_mm(&self) -> Self {
        match self {
            Self::Cm(val) => Self::Mm(val * 10.),
            Self::Mm(_) => *self,
            _ => self.as_cm().as_mm(),
        }
    }

    pub fn as_um(&self) -> Self {
        match self {
            Self::Mm(val) => Self::Um(val * 1000.),
            Self::Um(_) => *self,
            _ => self.as_mm().as_um(),
        }
    }

    pub fn as_nm(&self) -> Self {
        match self {
            Self::Um(val) => Self::Nm(val * 1000.),
            Self::Nm(_) => *self,
            _ => self.as_um().as_nm(),
        }
    }

    pub fn round_to(&self, places: f64) -> Self {
        match self {
            Self::In(val) => Self::In(round(*val, places)),
            Self::Ft(val) => Self::Ft(round(*val, places)),
            Self::Yd(val) => Self::Yd(round(*val, places)),
            Self::Mi(val) => Self::Mi(round(*val, places)),
            Self::M(val) => Self::M(round(*val, places)),
            Self::Km(val) => Self::Km(round(*val, places)),
            Self::Cm(val) => Self::Cm(round(*val, places)),
            Self::Mm(val) => Self::Mm(round(*val, places)),
            Self::Um(val) => Self::Um(round(*val, places)),
            Self::Nm(val) => Self::Nm(round(*val, places)),
        }
    }
}

impl Into<f64> for Length {
    fn into(self) -> f64 {
        match self {
            Length::In(val) => val,
            Length::Ft(val) => val,
            Length::Yd(val) => val,
            Length::Mi(val) => val,
            Length::M(val) => val,
            Length::Km(val) => val,
            Length::Cm(val) => val,
            Length::Mm(val) => val,
            Length::Um(val) => val,
            Length::Nm(val) => val,
        }
    }
}

fn round(val: f64, places: f64) -> f64 {
    (val * places).round() / places
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestData(Vec<Conversion>);

    #[derive(Debug, Serialize, Deserialize)]
    struct Conversion {
        meter: f64,
        kilometer: f64,
        centimeter: f64,
        millimeter: f64,
        micrometer: f64,
        nanometer: f64,
        mile: f64,
        yard: f64,
        foot: f64,
        inch: f64,
    }

    #[test]
    fn should_convert_meter_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // Convert from meter
            assert_eq!(
                Length::M(entry.meter).as_km().round_to(100.),
                Length::Km(entry.kilometer).round_to(100.)
            );

            assert_eq!(
                Length::M(entry.meter).as_cm().round_to(100.),
                Length::Cm(entry.centimeter).round_to(100.)
            );

            assert_eq!(
                Length::M(entry.meter).as_mm().round_to(100.),
                Length::Mm(entry.millimeter).round_to(100.)
            );

            assert_eq!(
                Length::M(entry.meter).as_um().round_to(100.),
                Length::Um(entry.micrometer).round_to(100.)
            );

            assert_eq!(
                Length::M(entry.meter).as_nm().round_to(100.),
                Length::Nm(entry.nanometer).round_to(100.)
            );

            assert_eq!(
                Length::M(entry.meter).as_mi().round_to(100.),
                Length::Mi(entry.mile).round_to(100.)
            );

            assert_eq!(
                Length::M(entry.meter).as_yd().round_to(100.),
                Length::Yd(entry.yard).round_to(100.)
            );

            assert_eq!(
                Length::M(entry.meter).as_ft().round_to(100.),
                Length::Ft(entry.foot).round_to(100.)
            );

            assert_eq!(
                Length::M(entry.meter).as_in().round_to(10.),
                Length::In(entry.inch).round_to(10.)
            );
        }
    }

    #[test]
    fn should_convert_kilometer_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            let km = Length::Km(entry.kilometer);

            // Convert from kilometer
            assert_eq!(
                km.as_m().round_to(100.),
                Length::M(entry.meter).round_to(100.)
            );

            assert_eq!(
                km.as_cm().round_to(100.),
                Length::Cm(entry.centimeter).round_to(100.)
            );

            assert_eq!(
                km.as_mm().round_to(100.),
                Length::Mm(entry.millimeter).round_to(100.)
            );

            assert_eq!(
                km.as_um().round_to(100.),
                Length::Um(entry.micrometer).round_to(100.)
            );

            assert_eq!(
                km.as_nm().round_to(100.),
                Length::Nm(entry.nanometer).round_to(100.)
            );

            assert_eq!(
                km.as_mi().round_to(100.),
                Length::Mi(entry.mile).round_to(100.)
            );

            assert_eq!(
                km.as_yd().round_to(100.),
                Length::Yd(entry.yard).round_to(100.)
            );

            assert_eq!(
                km.as_ft().round_to(100.),
                Length::Ft(entry.foot).round_to(100.)
            );

            assert_eq!(
                km.as_in().round_to(10.),
                Length::In(entry.inch).round_to(10.)
            );
        }
    }

    #[test]
    fn should_convert_centimeter_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // Convert from centimeter
            assert_eq!(
                Length::Cm(entry.centimeter).as_m().round_to(100.),
                Length::M(entry.meter).round_to(100.)
            );

            assert_eq!(
                Length::Cm(entry.centimeter).as_km().round_to(100.),
                Length::Km(entry.kilometer).round_to(100.)
            );

            assert_eq!(
                Length::Cm(entry.centimeter).as_mm().round_to(100.),
                Length::Mm(entry.millimeter).round_to(100.)
            );

            assert_eq!(
                Length::Cm(entry.centimeter).as_um().round_to(100.),
                Length::Um(entry.micrometer).round_to(100.)
            );

            assert_eq!(
                Length::Cm(entry.centimeter).as_nm().round_to(100.),
                Length::Nm(entry.nanometer).round_to(100.)
            );

            assert_eq!(
                Length::Cm(entry.centimeter).as_mi().round_to(100.),
                Length::Mi(entry.mile).round_to(100.)
            );

            assert_eq!(
                Length::Cm(entry.centimeter).as_yd().round_to(100.),
                Length::Yd(entry.yard).round_to(100.)
            );

            assert_eq!(
                Length::Cm(entry.centimeter).as_ft().round_to(100.),
                Length::Ft(entry.foot).round_to(100.)
            );

            assert_eq!(
                Length::Cm(entry.centimeter).as_in().round_to(10.),
                Length::In(entry.inch).round_to(10.)
            );
        }
    }

    #[test]
    fn should_convert_millimeter_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // Convert from millimeter
            assert_eq!(
                Length::Mm(entry.millimeter).as_m().round_to(100.),
                Length::M(entry.meter).round_to(100.)
            );

            assert_eq!(
                Length::Mm(entry.millimeter).as_km().round_to(100.),
                Length::Km(entry.kilometer).round_to(100.)
            );

            assert_eq!(
                Length::Mm(entry.millimeter).as_cm().round_to(100.),
                Length::Cm(entry.centimeter).round_to(100.)
            );

            assert_eq!(
                Length::Mm(entry.millimeter).as_um().round_to(100.),
                Length::Um(entry.micrometer).round_to(100.)
            );

            assert_eq!(
                Length::Mm(entry.millimeter).as_nm().round_to(100.),
                Length::Nm(entry.nanometer).round_to(100.)
            );

            assert_eq!(
                Length::Mm(entry.millimeter).as_mi().round_to(100.),
                Length::Mi(entry.mile).round_to(100.)
            );

            assert_eq!(
                Length::Mm(entry.millimeter).as_yd().round_to(100.),
                Length::Yd(entry.yard).round_to(100.)
            );

            assert_eq!(
                Length::Mm(entry.millimeter).as_ft().round_to(100.),
                Length::Ft(entry.foot).round_to(100.)
            );

            assert_eq!(
                Length::Mm(entry.millimeter).as_in().round_to(10.),
                Length::In(entry.inch).round_to(10.)
            );
        }
    }

    #[test]
    fn should_convert_micrometer_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // Convert from micrometer
            assert_eq!(
                Length::Um(entry.micrometer).as_m().round_to(100.),
                Length::M(entry.meter).round_to(100.)
            );

            assert_eq!(
                Length::Um(entry.micrometer).as_km().round_to(100.),
                Length::Km(entry.kilometer).round_to(100.)
            );

            assert_eq!(
                Length::Um(entry.micrometer).as_cm().round_to(100.),
                Length::Cm(entry.centimeter).round_to(100.)
            );

            assert_eq!(
                Length::Um(entry.micrometer).as_mm().round_to(100.),
                Length::Mm(entry.millimeter).round_to(100.)
            );

            assert_eq!(
                Length::Um(entry.micrometer).as_nm().round_to(100.),
                Length::Nm(entry.nanometer).round_to(100.)
            );

            assert_eq!(
                Length::Um(entry.micrometer).as_mi().round_to(100.),
                Length::Mi(entry.mile).round_to(100.)
            );

            assert_eq!(
                Length::Um(entry.micrometer).as_yd().round_to(100.),
                Length::Yd(entry.yard).round_to(100.)
            );

            assert_eq!(
                Length::Um(entry.micrometer).as_ft().round_to(100.),
                Length::Ft(entry.foot).round_to(100.)
            );

            assert_eq!(
                Length::Um(entry.micrometer).as_in().round_to(10.),
                Length::In(entry.inch).round_to(10.)
            );
        }
    }

    #[test]
    fn should_convert_nanometer_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // Convert from nanometer
            assert_eq!(
                Length::Nm(entry.nanometer).as_m().round_to(100.),
                Length::M(entry.meter).round_to(100.)
            );

            assert_eq!(
                Length::Nm(entry.nanometer).as_km().round_to(100.),
                Length::Km(entry.kilometer).round_to(100.)
            );

            assert_eq!(
                Length::Nm(entry.nanometer).as_cm().round_to(100.),
                Length::Cm(entry.centimeter).round_to(100.)
            );

            assert_eq!(
                Length::Nm(entry.nanometer).as_mm().round_to(100.),
                Length::Mm(entry.millimeter).round_to(100.)
            );

            assert_eq!(
                Length::Nm(entry.nanometer).as_um().round_to(100.),
                Length::Um(entry.micrometer).round_to(100.)
            );

            assert_eq!(
                Length::Nm(entry.nanometer).as_mi().round_to(100.),
                Length::Mi(entry.mile).round_to(100.)
            );

            assert_eq!(
                Length::Nm(entry.nanometer).as_yd().round_to(100.),
                Length::Yd(entry.yard).round_to(100.)
            );

            assert_eq!(
                Length::Nm(entry.nanometer).as_ft().round_to(100.),
                Length::Ft(entry.foot).round_to(100.)
            );

            assert_eq!(
                Length::Nm(entry.nanometer).as_in().round_to(10.),
                Length::In(entry.inch).round_to(10.)
            );
        }
    }
}
