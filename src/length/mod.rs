mod units;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Length {
    In(f32),
    Ft(f32),
    Yrd(f32),
    Mile(f32),
    MM(f32),
    CM(f32),
    M(f32),
    KM(f32),
}
