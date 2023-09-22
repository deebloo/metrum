mod ops;
mod units;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Length {
    In(f32),
    Ft(f32),
    Yrd(f32),
    // Mile(f32),
    // MM(f32),
    // CM(f32),
    // M(f32),
    // KM(f32),
}

impl Into<f32> for Length {
    fn into(self) -> f32 {
        match self {
            Self::In(val) => val,
            Self::Ft(val) => val,
            Self::Yrd(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};
    use std::fs;

    use super::*;

    #[test]
    fn should_convert_to_f32() {
        let inch: f32 = Length::In(1.).into();
        let feet: f32 = Length::Ft(2.).into();
        let yards: f32 = Length::Yrd(3.).into();

        assert_eq!(inch, 1.);
        assert_eq!(feet, 2.);
        assert_eq!(yards, 3.);
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct TestData(Vec<LengthData>);

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    struct LengthData {
        inches: f32,
        feet: f32,
        yards: f32,
        mm: f32,
        cm: f32,
        m: f32,
        km: f32,
        miles: f32,
    }

    #[test]
    fn should_convert() {
        let data_string = fs::read_to_string("data/lengths.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            assert_eq!(Length::In(entry.inches), Length::Ft(entry.feet).as_inches())
        }
    }
}
