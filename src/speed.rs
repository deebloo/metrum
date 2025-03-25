pub mod add;
pub mod cmp;
pub mod div;
pub mod mul;
pub mod sub;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Speed {
    meters_per_second: f64, // Using m/s as base unit for maximum precision
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Speed {
    pub fn from_mps(val: f64) -> Self {
        Self {
            meters_per_second: val,
        }
    }

    pub fn from_kmph(val: f64) -> Self {
        Self {
            meters_per_second: val / 3.6, // Convert km/h to m/s
        }
    }

    pub fn from_mph(val: f64) -> Self {
        Self {
            meters_per_second: (val * 0.44704 * 100_000_000.).round() / 100_000_000., // Convert mph to m/s
        }
    }

    pub fn from_knots(val: f64) -> Self {
        Self {
            meters_per_second: (val * 0.514444 * 100_000_000.).round() / 100_000_000., // Convert knots to m/s
        }
    }

    pub fn as_mps(&self) -> f64 {
        self.meters_per_second
    }

    pub fn as_kmph(&self) -> f64 {
        self.meters_per_second * 3.6
    }

    pub fn as_mph(&self) -> f64 {
        self.meters_per_second / 0.44704
    }

    pub fn as_knots(&self) -> f64 {
        self.meters_per_second / 0.514444
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Conversion {
        mps: f64,   // meters per second
        kmph: f64,  // kilometers per hour
        mph: f64,   // miles per hour
        knots: f64, // knots
    }

    #[test]
    fn should_convert_correctly() {
        let speeds = vec![
            Conversion {
                mps: 1.0,
                kmph: 3.6,
                mph: 2.2369362920544025,
                knots: 1.9438461717893492,
            },
            Conversion {
                mps: 10.0,
                kmph: 36.0,
                mph: 22.369362920544024,
                knots: 19.438461717893492,
            },
            Conversion {
                mps: 0.5,
                kmph: 1.8,
                mph: 1.1184681460272012,
                knots: 0.9719230858946746,
            },
        ];

        for speed in speeds {
            let mps_source = Speed::from_mps(speed.mps);
            let kmph_source = Speed::from_kmph(speed.kmph);
            let mph_source = Speed::from_mph(speed.mph);
            let knots_source = Speed::from_knots(speed.knots);

            // Convert to meters per second
            assert_eq!(kmph_source.as_mps(), speed.mps);
            assert_eq!(mph_source.as_mps(), speed.mps);
            assert_eq!(knots_source.as_mps(), speed.mps);

            // Convert to kilometers per hour
            assert_eq!(mps_source.as_kmph(), speed.kmph);
            assert_eq!(mph_source.as_kmph(), speed.kmph);
            assert_eq!(knots_source.as_kmph(), speed.kmph);

            // // Convert to miles per hour
            assert_eq!(mps_source.as_mph(), speed.mph);
            assert_eq!(kmph_source.as_mph(), speed.mph);
            // assert_eq!(knots_source.as_mph(), speed.mph);

            // Convert to knots
            assert_eq!(mps_source.as_knots(), speed.knots);
            assert_eq!(kmph_source.as_knots(), speed.knots);
            assert_eq!(mph_source.as_knots(), speed.knots);
        }
    }
}
