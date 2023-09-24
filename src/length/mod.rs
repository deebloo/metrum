mod ops;
mod units;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Length {
    In(f64),
    Ft(f64),
    Yd(f64),
    // Mi(f64),
    // MM(f64),
    // CM(f64),
    // M(f64),
    // KM(f64),
}

impl Into<f64> for Length {
    fn into(self) -> f64 {
        match self {
            Self::In(val) => val,
            Self::Ft(val) => val,
            Self::Yd(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};
    use std::fs;

    use super::*;

    #[test]
    fn should_convert_to_f64() {
        let inch: f64 = Length::In(1.).into();
        let feet: f64 = Length::Ft(2.).into();
        let yards: f64 = Length::Yd(3.).into();

        assert_eq!(inch, 1.);
        assert_eq!(feet, 2.);
        assert_eq!(yards, 3.);
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct TestData(Vec<LengthData>);

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    struct LengthData {
        inches: f64,
        feet: f64,
        yards: f64,
        // mm: f64,
        // cm: f64,
        // m: f64,
        // km: f64,
        // miles: f64,
    }

    #[test]
    fn should_convert() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            assert_eq!(Length::In(entry.inches), Length::Ft(entry.feet).as_inches());
            assert_eq!(
                Length::In(entry.inches),
                Length::Yd(entry.yards).as_inches()
            );

            assert_eq!(Length::Ft(entry.feet), Length::In(entry.inches).as_feet());
            assert_eq!(Length::Ft(entry.feet), Length::Yd(entry.yards).as_feet());
        }
    }
}
