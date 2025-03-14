use crate::temp::Temp;

pub trait IntoTemp {
    fn into_tempc(self) -> Temp;
    fn into_tempf(self) -> Temp;
    fn into_tempk(self) -> Temp;
}

impl IntoTemp for f32 {
    fn into_tempc(self) -> Temp {
        Temp::C(self)
    }

    fn into_tempf(self) -> Temp {
        Temp::F(self)
    }

    fn into_tempk(self) -> Temp {
        Temp::K(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_as_c() {
        assert_eq!(100.0.into_tempc(), Temp::C(100.0))
    }

    #[test]
    fn should_create_as_f() {
        assert_eq!(100.0.into_tempf(), Temp::F(100.0))
    }

    #[test]
    fn should_create_as_k() {
        assert_eq!(100.0.into_tempk(), Temp::K(100.0))
    }
}
