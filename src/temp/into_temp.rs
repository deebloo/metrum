use crate::temp::Temp;

pub trait IntoTemp {
    fn into_temp_c(self) -> Temp;
    fn into_temp_f(self) -> Temp;
    fn into_temp_k(self) -> Temp;
}

impl IntoTemp for f32 {
    fn into_temp_c(self) -> Temp {
        Temp::C(self)
    }

    fn into_temp_f(self) -> Temp {
        Temp::F(self)
    }

    fn into_temp_k(self) -> Temp {
        Temp::K(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_as_c() {
        assert_eq!(100.0.into_temp_c(), Temp::C(100.0))
    }

    #[test]
    fn should_create_as_f() {
        assert_eq!(100.0.into_temp_f(), Temp::F(100.0))
    }

    #[test]
    fn should_create_as_k() {
        assert_eq!(100.0.into_temp_k(), Temp::K(100.0))
    }
}
