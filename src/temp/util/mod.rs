pub trait RoundTo<T> {
    fn round_to(self, places: T) -> T;
}

impl RoundTo<f64> for f64 {
    fn round_to(self, places: f64) -> f64 {
        (self * places).round() / places
    }
}

impl RoundTo<f32> for f32 {
    fn round_to(self, places: f32) -> f32 {
        (self * places).round() / places
    }
}
