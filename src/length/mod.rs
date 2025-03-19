mod add;
mod cmp;
mod div;
mod mul;
mod sub;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Length {
    nanometers: f64,
}

impl Length {
    pub fn from_m(val: f64) -> Self {
        Self {
            nanometers: val * 1_000_000_000.0,
        }
    }

    pub fn from_km(val: f64) -> Self {
        Self {
            nanometers: val * 1_000_000_000_000.0,
        }
    }

    pub fn from_mm(val: f64) -> Self {
        Self {
            nanometers: val * 1_000_000.0,
        }
    }

    pub fn from_um(val: f64) -> Self {
        Self {
            nanometers: val * 1_000.0,
        }
    }

    pub fn from_nm(val: f64) -> Self {
        Self { nanometers: val }
    }

    pub fn from_in(val: f64) -> Self {
        Self {
            nanometers: val * 25_400_000.0,
        }
    }

    pub fn as_m(&self) -> f64 {
        self.nanometers / 1_000_000_000.0
    }

    pub fn as_km(&self) -> f64 {
        self.nanometers / 1_000_000_000_000.0
    }

    pub fn as_mm(&self) -> f64 {
        self.nanometers / 1_000_000.0
    }

    pub fn as_um(&self) -> f64 {
        self.nanometers / 1_000.0
    }

    pub fn as_nm(&self) -> f64 {
        self.nanometers
    }

    pub fn as_in(&self) -> f64 {
        self.nanometers / 25_400_000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Conversion {
        m: f64,    // meters
        km: f64,   // kilometers
        mm: f64,   // millimeters
        um: f64,   // micrometers
        nm: f64,   // nanometers
        inch: f64, // inches
    }

    #[test]
    fn should_convert_correctly() {
        let lengths = vec![
            Conversion {
                m: 1.0,
                km: 0.001,
                mm: 1000.0,
                um: 1_000_000.0,
                nm: 1_000_000_000.0,
                inch: 39.37007874015748,
            },
            Conversion {
                m: 1000.0,
                km: 1.0,
                mm: 1_000_000.0,
                um: 1_000_000_000.0,
                nm: 1_000_000_000_000.0,
                inch: 39370.07874015748,
            },
            Conversion {
                m: 0.001,
                km: 0.000001,
                mm: 1.0,
                um: 1000.0,
                nm: 1_000_000.0,
                inch: 0.03937007874015748,
            },
        ];

        for length in lengths {
            let m_source = Length::from_m(length.m);
            let km_source = Length::from_km(length.km);
            let mm_source = Length::from_mm(length.mm);
            let um_source = Length::from_um(length.um);
            let nm_source = Length::from_nm(length.nm);
            let inchsource = Length::from_in(length.inch);

            // Convert to meters
            assert_eq!(km_source.as_m(), length.m);
            assert_eq!(mm_source.as_m(), length.m);
            assert_eq!(um_source.as_m(), length.m);
            assert_eq!(nm_source.as_m(), length.m);
            assert_eq!(inchsource.as_m(), length.m);

            // Convert to kilometers
            assert_eq!(m_source.as_km(), length.km);
            assert_eq!(mm_source.as_km(), length.km);
            assert_eq!(um_source.as_km(), length.km);
            assert_eq!(nm_source.as_km(), length.km);
            assert_eq!(inchsource.as_km(), length.km);

            // Convert to millimeters
            assert_eq!(m_source.as_mm(), length.mm);
            assert_eq!(km_source.as_mm(), length.mm);
            assert_eq!(um_source.as_mm(), length.mm);
            assert_eq!(nm_source.as_mm(), length.mm);
            assert_eq!(inchsource.as_mm(), length.mm);

            // Convert to micrometers
            assert_eq!(m_source.as_um(), length.um);
            assert_eq!(km_source.as_um(), length.um);
            assert_eq!(mm_source.as_um(), length.um);
            assert_eq!(nm_source.as_um(), length.um);
            assert_eq!(inchsource.as_um(), length.um);

            // Convert to nanometers
            assert_eq!(m_source.as_nm(), length.nm);
            assert_eq!(km_source.as_nm(), length.nm);
            assert_eq!(mm_source.as_nm(), length.nm);
            assert_eq!(um_source.as_nm(), length.nm);
            assert_eq!(inchsource.as_nm(), length.nm);

            // Convert to inches
            assert_eq!(m_source.as_in(), length.inch);
            assert_eq!(km_source.as_in(), length.inch);
            assert_eq!(mm_source.as_in(), length.inch);
            assert_eq!(um_source.as_in(), length.inch);
            assert_eq!(nm_source.as_in(), length.inch);
        }
    }
}
