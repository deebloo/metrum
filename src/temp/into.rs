use crate::temp::Temp;

pub trait IntoTemp {
    fn into_tempc(self) -> Temp;
    fn into_tempf(self) -> Temp;
    fn into_tempk(self) -> Temp;
}

impl IntoTemp for f64 {
    fn into_tempc(self) -> Temp {
        Temp::from_c(self)
    }

    fn into_tempf(self) -> Temp {
        Temp::from_f(self)
    }

    fn into_tempk(self) -> Temp {
        Temp::from_k(self)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn should_create_as_c() {
    //     assert_eq!(100.0.into_tempc(), Temp::from_c(100.0))
    // }

    // #[test]
    // fn should_create_as_f() {
    //     assert_eq!(100.0.into_tempf(), Temp::from_f(100.0))
    // }

    // #[test]
    // fn should_create_as_k() {
    //     assert_eq!(100.0.into_tempk(), Temp::from_k(100.0))
    // }
}
