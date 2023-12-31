use crate::temp::Temp;

impl PartialOrd for Temp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let source: f64 = (*self).as_k().into();
        let target: f64 = (*other).as_k().into();

        let mut res = std::cmp::Ordering::Equal;

        if source > target {
            res = std::cmp::Ordering::Greater;
        } else if source < target {
            res = std::cmp::Ordering::Less;
        }

        Some(res)
    }
}

impl PartialEq for Temp {
    fn eq(&self, other: &Self) -> bool {
        let source: f64 = (*self).as_k().into();
        let target: f64 = (*other).as_k().into();

        source == target
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use std::fs;

    use super::*;

    #[test]
    fn equal() {
        assert_eq!(Temp::F(86.) == Temp::C(30.), true);
        assert_eq!(Temp::F(86.) == Temp::K(303.15), true);
        assert_eq!(Temp::C(30.) == Temp::K(303.15), true);
    }

    #[test]
    fn gte() {
        assert_eq!(Temp::F(85.) >= Temp::F(80.), true);
    }

    #[test]
    fn lte() {
        assert_eq!(Temp::F(85.) <= Temp::F(112.), true);
        assert_eq!(Temp::F(85.) <= Temp::F(87.), true);
    }

    #[test]
    fn should_be_gt() {
        assert_eq!(Temp::F(86.) > Temp::C(0.), true);
        assert_eq!(Temp::F(86.) > Temp::C(100.), false);
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct TestData(Vec<Conversion>);

    #[derive(Debug, Serialize, Deserialize)]
    struct Conversion {
        celsius: f64,
        fahrenheit: f64,
        kelvin: f64,
    }

    #[test]
    fn should_convert_correctly() {
        let data_string = fs::read_to_string("data/temps.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            assert!(Temp::F(entry.fahrenheit) == Temp::C(entry.celsius));
            assert!(Temp::F(entry.fahrenheit) == Temp::K(entry.kelvin));

            assert!(Temp::C(entry.celsius) == Temp::F(entry.fahrenheit));
            assert!(Temp::C(entry.celsius) == Temp::K(entry.kelvin));

            assert!(Temp::K(entry.kelvin) == Temp::F(entry.fahrenheit));
            assert!(Temp::K(entry.kelvin) == Temp::C(entry.celsius));
        }
    }
}
