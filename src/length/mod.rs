#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Length {
    In(f32), // Inches
    Ft(f32), // Feet
    Yd(f32), // Yards
    Mi(f32), // Miles
    M(f32),  // Meters
    Km(f32), // Kilometers
    Cm(f32), // Centimeters
    Mm(f32), // Millimeters
    Um(f32), // Micrometers
    Nm(f32), // Nanometers
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
            Self::Cm(val) => Self::In(val * 0.393701),
            Self::Mm(val) => Self::In(val * 0.0393701),
            Self::Um(val) => Self::In(val * 0.0000393701),
            Self::Nm(val) => Self::In(val * 0.0000000393701),
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
            Self::Cm(val) => Self::Ft(val * 0.0328084),
            Self::Mm(val) => Self::Ft(val * 0.00328084),
            Self::Um(val) => Self::Ft(val * 0.00000328084),
            Self::Nm(val) => Self::Ft(val * 0.00000000328084),
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
            Self::Cm(val) => Self::Yd(val * 0.0109361),
            Self::Mm(val) => Self::Yd(val * 0.00109361),
            Self::Um(val) => Self::Yd(val * 0.00000109361),
            Self::Nm(val) => Self::Yd(val * 0.00000000109361),
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
            Self::Mi(val) => Self::Cm(val * 160934.),
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

    pub fn round_to(&self, places: f32) -> Self {
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

impl Into<f32> for Length {
    fn into(self) -> f32 {
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

fn round(val: f32, places: f32) -> f32 {
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
        meter: f32,
        kilometer: f32,
        centimeter: f32,
        millimeter: f32,
        micrometer: f32,
        nanometer: f32,
        mile: f32,
        yard: f32,
        foot: f32,
        inch: f32,
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
                Length::M(entry.meter).as_in().round_to(100.),
                Length::In(entry.inch).round_to(100.)
            );
        }
    }

    #[test]
    fn should_convert_kilometer_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // Convert from kilometer
            assert_eq!(
                Length::Km(entry.kilometer).as_m().round_to(100.),
                Length::M(entry.meter).round_to(100.)
            );

            assert_eq!(
                Length::Km(entry.kilometer).as_cm().round_to(100.),
                Length::Cm(entry.centimeter).round_to(100.)
            );

            assert_eq!(
                Length::Km(entry.kilometer).as_mm().round_to(100.),
                Length::Mm(entry.millimeter).round_to(100.)
            );

            assert_eq!(
                Length::Km(entry.kilometer).as_um().round_to(100.),
                Length::Um(entry.micrometer).round_to(100.)
            );

            assert_eq!(
                Length::Km(entry.kilometer).as_nm().round_to(100.),
                Length::Nm(entry.nanometer).round_to(100.)
            );

            assert_eq!(
                Length::Km(entry.kilometer).as_mi().round_to(100.),
                Length::Mi(entry.mile).round_to(100.)
            );

            assert_eq!(
                Length::Km(entry.kilometer).as_yd().round_to(100.),
                Length::Yd(entry.yard).round_to(100.)
            );

            assert_eq!(
                Length::Km(entry.kilometer).as_ft().round_to(100.),
                Length::Ft(entry.foot).round_to(100.)
            );

            assert_eq!(
                Length::Km(entry.kilometer).as_in().round_to(100.),
                Length::In(entry.inch).round_to(100.)
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
                Length::Cm(entry.centimeter).as_in().round_to(100.),
                Length::In(entry.inch).round_to(100.)
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
                Length::Mm(entry.millimeter).as_in().round_to(100.),
                Length::In(entry.inch).round_to(100.)
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
                Length::Um(entry.micrometer).as_in().round_to(100.),
                Length::In(entry.inch).round_to(100.)
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
                Length::Nm(entry.nanometer).as_in().round_to(100.),
                Length::In(entry.inch).round_to(100.)
            );
        }
    }

    #[test]
    fn should_convert_inch_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // Convert from inch
            assert_eq!(
                Length::In(entry.inch).as_m().round_to(100.),
                Length::M(entry.meter).round_to(100.)
            );

            assert_eq!(
                Length::In(entry.inch).as_km().round_to(100.),
                Length::Km(entry.kilometer).round_to(100.)
            );

            assert_eq!(
                Length::In(entry.inch).as_cm().round_to(100.),
                Length::Cm(entry.centimeter).round_to(100.)
            );

            assert_eq!(
                Length::In(entry.inch).as_mm().round_to(100.),
                Length::Mm(entry.millimeter).round_to(100.)
            );

            assert_eq!(
                Length::In(entry.inch).as_um().round_to(100.),
                Length::Um(entry.micrometer).round_to(100.)
            );

            assert_eq!(
                Length::In(entry.inch).as_nm().round_to(100.),
                Length::Nm(entry.nanometer).round_to(100.)
            );

            assert_eq!(
                Length::In(entry.inch).as_mi().round_to(100.),
                Length::Mi(entry.mile).round_to(100.)
            );

            assert_eq!(
                Length::In(entry.inch).as_yd().round_to(100.),
                Length::Yd(entry.yard).round_to(100.)
            );

            assert_eq!(
                Length::In(entry.inch).as_ft().round_to(100.),
                Length::Ft(entry.foot).round_to(100.)
            );
        }
    }

    #[test]
    fn should_convert_foot_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // Convert from foot
            assert_eq!(
                Length::Ft(entry.foot).as_m().round_to(100.),
                Length::M(entry.meter).round_to(100.)
            );

            assert_eq!(
                Length::Ft(entry.foot).as_km().round_to(100.),
                Length::Km(entry.kilometer).round_to(100.)
            );

            assert_eq!(
                Length::Ft(entry.foot).as_cm().round_to(100.),
                Length::Cm(entry.centimeter).round_to(100.)
            );

            assert_eq!(
                Length::Ft(entry.foot).as_mm().round_to(100.),
                Length::Mm(entry.millimeter).round_to(100.)
            );

            assert_eq!(
                Length::Ft(entry.foot).as_um().round_to(100.),
                Length::Um(entry.micrometer).round_to(100.)
            );

            assert_eq!(
                Length::Ft(entry.foot).as_nm().round_to(100.),
                Length::Nm(entry.nanometer).round_to(100.)
            );

            assert_eq!(
                Length::Ft(entry.foot).as_mi().round_to(100.),
                Length::Mi(entry.mile).round_to(100.)
            );

            assert_eq!(
                Length::Ft(entry.foot).as_yd().round_to(100.),
                Length::Yd(entry.yard).round_to(100.)
            );

            assert_eq!(
                Length::Ft(entry.foot).as_in().round_to(100.),
                Length::In(entry.inch).round_to(100.)
            );
        }
    }

    #[test]
    fn should_convert_yard_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // Convert from yard
            assert_eq!(
                Length::Yd(entry.yard).as_m().round_to(100.),
                Length::M(entry.meter).round_to(100.)
            );

            assert_eq!(
                Length::Yd(entry.yard).as_km().round_to(100.),
                Length::Km(entry.kilometer).round_to(100.)
            );

            assert_eq!(
                Length::Yd(entry.yard).as_cm().round_to(100.),
                Length::Cm(entry.centimeter).round_to(100.)
            );

            assert_eq!(
                Length::Yd(entry.yard).as_mm().round_to(100.),
                Length::Mm(entry.millimeter).round_to(100.)
            );

            assert_eq!(
                Length::Yd(entry.yard).as_um().round_to(100.),
                Length::Um(entry.micrometer).round_to(100.)
            );

            assert_eq!(
                Length::Yd(entry.yard).as_nm().round_to(100.),
                Length::Nm(entry.nanometer).round_to(100.)
            );

            assert_eq!(
                Length::Yd(entry.yard).as_mi().round_to(100.),
                Length::Mi(entry.mile).round_to(100.)
            );

            assert_eq!(
                Length::Yd(entry.yard).as_in().round_to(100.),
                Length::In(entry.inch).round_to(100.)
            );

            assert_eq!(
                Length::Yd(entry.yard).as_ft().round_to(100.),
                Length::Ft(entry.foot).round_to(100.)
            );
        }
    }

    #[test]
    fn should_convert_mile_to_others() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // Convert from mile
            assert_eq!(
                Length::Mi(entry.mile).as_m().round_to(100.),
                Length::M(entry.meter).round_to(100.)
            );

            assert_eq!(
                Length::Mi(entry.mile).as_km().round_to(100.),
                Length::Km(entry.kilometer).round_to(100.)
            );

            assert_eq!(
                Length::Mi(entry.mile).as_cm().round_to(100.),
                Length::Cm(entry.centimeter).round_to(100.)
            );

            assert_eq!(
                Length::Mi(entry.mile).as_mm().round_to(100.),
                Length::Mm(entry.millimeter).round_to(100.)
            );

            assert_eq!(
                Length::Mi(entry.mile).as_um().round_to(100.),
                Length::Um(entry.micrometer).round_to(100.)
            );

            assert_eq!(
                Length::Mi(entry.mile).as_nm().round_to(100.),
                Length::Nm(entry.nanometer).round_to(100.)
            );

            assert_eq!(
                Length::Mi(entry.mile).as_yd().round_to(100.),
                Length::Yd(entry.yard).round_to(100.)
            );

            assert_eq!(
                Length::Mi(entry.mile).as_in().round_to(100.),
                Length::In(entry.inch).round_to(100.)
            );

            assert_eq!(
                Length::Mi(entry.mile).as_ft().round_to(100.),
                Length::Ft(entry.foot).round_to(100.)
            );
        }
    }
}
