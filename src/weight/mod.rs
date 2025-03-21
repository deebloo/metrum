pub mod add;
pub mod cmp;
pub mod div;
pub mod mul;
pub mod sub;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Weight {
    nanograms: f64, // Using nanograms as base unit for maximum precision
}

impl Weight {
    pub fn from_kg(val: f64) -> Self {
        Self {
            nanograms: val * 1_000_000_000_000.0,
        }
    }

    pub fn from_g(val: f64) -> Self {
        Self {
            nanograms: val * 1_000_000_000.0,
        }
    }

    pub fn from_mg(val: f64) -> Self {
        Self {
            nanograms: val * 1_000_000.0,
        }
    }

    pub fn from_mcg(val: f64) -> Self {
        Self {
            nanograms: val * 1_000.0,
        }
    }

    pub fn from_ng(val: f64) -> Self {
        Self { nanograms: val }
    }

    pub fn from_lbs(val: f64) -> Self {
        Self {
            nanograms: (val * 453592370000. * 1_000.).round() / 1_000.,
        }
    }

    pub fn as_kg(&self) -> f64 {
        self.nanograms / 1_000_000_000_000.0
    }

    pub fn as_g(&self) -> f64 {
        self.nanograms / 1_000_000_000.0
    }

    pub fn as_mg(&self) -> f64 {
        self.nanograms / 1_000_000.0
    }

    pub fn as_mcg(&self) -> f64 {
        self.nanograms / 1_000.0
    }

    pub fn as_ng(&self) -> f64 {
        self.nanograms
    }

    pub fn as_lbs(&self) -> f64 {
        self.nanograms / 453592370000.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Conversion {
        kg: f64,  // kilograms
        g: f64,   // grams
        mg: f64,  // milligrams
        mcg: f64, // micrograms
        ng: f64,  // nanograms
        lbs: f64, // pounds
    }

    #[test]
    fn should_convert_correctly() {
        let weights = vec![
            Conversion {
                kg: 1.0,
                g: 1000.0,
                mg: 1_000_000.0,
                mcg: 1_000_000_000.0,
                ng: 1_000_000_000_000.0,
                lbs: 2.2046226218487757, // 1 kg in lbs
            },
            Conversion {
                kg: 2.0,
                g: 2000.0,
                mg: 2_000_000.0,
                mcg: 2_000_000_000.0,
                ng: 2_000_000_000_000.0,
                lbs: 4.409245243697551, // 2 kg in lbs
            },
            Conversion {
                kg: 0.5,
                g: 500.0,
                mg: 500_000.0,
                mcg: 500_000_000.0,
                ng: 500_000_000_000.0,
                lbs: 1.1023113109243878, // 0.5 kg in lbs
            },
        ];

        for weight in weights {
            let kg_source = Weight::from_kg(weight.kg);
            let g_source = Weight::from_g(weight.g);
            let mg_source = Weight::from_mg(weight.mg);
            let mcg_source = Weight::from_mcg(weight.mcg);
            let ng_source = Weight::from_ng(weight.ng);
            let lbs_source = Weight::from_lbs(weight.lbs);

            // Convert to kilograms
            assert_eq!(g_source.as_kg(), weight.kg);
            assert_eq!(mg_source.as_kg(), weight.kg);
            assert_eq!(mcg_source.as_kg(), weight.kg);
            assert_eq!(ng_source.as_kg(), weight.kg);
            assert_eq!(lbs_source.as_kg(), weight.kg);

            // Convert to grams
            assert_eq!(kg_source.as_g(), weight.g);
            assert_eq!(mg_source.as_g(), weight.g);
            assert_eq!(mcg_source.as_g(), weight.g);
            assert_eq!(ng_source.as_g(), weight.g);
            assert_eq!(lbs_source.as_g(), weight.g);

            // Convert to milligrams
            assert_eq!(kg_source.as_mg(), weight.mg);
            assert_eq!(g_source.as_mg(), weight.mg);
            assert_eq!(mcg_source.as_mg(), weight.mg);
            assert_eq!(ng_source.as_mg(), weight.mg);
            assert_eq!(lbs_source.as_mg(), weight.mg);

            // Convert to micrograms
            assert_eq!(kg_source.as_mcg(), weight.mcg);
            assert_eq!(g_source.as_mcg(), weight.mcg);
            assert_eq!(mg_source.as_mcg(), weight.mcg);
            assert_eq!(ng_source.as_mcg(), weight.mcg);
            assert_eq!(lbs_source.as_mcg(), weight.mcg);

            // Convert to nanograms
            assert_eq!(kg_source.as_ng(), weight.ng);
            assert_eq!(g_source.as_ng(), weight.ng);
            assert_eq!(mg_source.as_ng(), weight.ng);
            assert_eq!(mcg_source.as_ng(), weight.ng);
            assert_eq!(lbs_source.as_ng(), weight.ng);

            // Convert to pounds
            assert_eq!(kg_source.as_lbs(), weight.lbs);
            assert_eq!(g_source.as_lbs(), weight.lbs);
            assert_eq!(mg_source.as_lbs(), weight.lbs);
            assert_eq!(mcg_source.as_lbs(), weight.lbs);
            assert_eq!(ng_source.as_lbs(), weight.lbs);
        }
    }
}
