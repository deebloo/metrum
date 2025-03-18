use super::Temp;

impl From<Temp> for f32 {
    fn from(item: Temp) -> f32 {
        match item {
            Temp::C(val) => val,
            Temp::F(val) => val,
            Temp::K(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_f32() {
        let c: f32 = Temp::C(0.).into();
        let f: f32 = Temp::F(32.).into();
        let k: f32 = Temp::K(273.15).into();

        assert_eq!(c, 0.);
        assert_eq!(f, 32.);
        assert_eq!(k, 273.15);
    }
}
