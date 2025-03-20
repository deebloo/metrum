use super::Temp;

impl PartialEq for Temp {
    fn eq(&self, other: &Self) -> bool {
        let source = self.as_k();
        let target = other.as_k();

        let mut epsilon: f64 = f64::EPSILON;

        if (source - target).abs() <= epsilon {
            return true;
        }

        epsilon = if source > target {
            epsilon * source.abs()
        } else {
            epsilon * target.abs()
        };

        (source - target).abs() <= epsilon
    }
}

impl PartialOrd for Temp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let source = self.as_k();
        let target = other.as_k();

        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if source > target {
            Some(std::cmp::Ordering::Greater)
        } else if source < target {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal() {
        assert_eq!(Temp::from_f(86.) == Temp::from_c(30.), true);
        assert_eq!(Temp::from_f(86.) == Temp::from_k(303.15), true);
        assert_eq!(Temp::from_c(30.) == Temp::from_k(303.15), true);
    }

    #[test]
    fn gte() {
        assert_eq!(Temp::from_f(85.) >= Temp::from_f(80.), true);
    }

    #[test]
    fn lte() {
        assert_eq!(Temp::from_f(85.) <= Temp::from_f(112.), true);
        assert_eq!(Temp::from_f(85.) <= Temp::from_f(87.), true);
    }

    #[test]
    fn should_be_gt() {
        assert_eq!(Temp::from_f(86.) > Temp::from_c(0.), true);
        assert_eq!(Temp::from_f(86.) > Temp::from_c(100.), false);
    }
}
