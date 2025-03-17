use crate::temp::Temp;

impl PartialOrd for Temp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let source = (*self).as_k();
        let target = (*other).as_k();

        let mut res = std::cmp::Ordering::Equal;

        if source > target {
            res = std::cmp::Ordering::Greater;
        } else if source < target {
            res = std::cmp::Ordering::Less;
        }

        Some(res)
    }
}

impl PartialEq for Temp {
    fn eq(&self, other: &Self) -> bool {
        let source = (*self).as_k();
        let target = (*other).as_k();

        source == target
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn equal() {
//         assert_eq!(Temp::from_f(86.) == Temp::from_c(30.), true);
//         assert_eq!(Temp::from_f(86.) == Temp::from_k(303.15), true);
//         assert_eq!(Temp::from_c(30.) == Temp::from_k(303.15), true);
//     }

//     #[test]
//     fn gte() {
//         assert_eq!(Temp::from_f(85.) >= Temp::from_f(80.), true);
//     }

//     #[test]
//     fn lte() {
//         assert_eq!(Temp::from_f(85.) <= Temp::from_f(112.), true);
//         assert_eq!(Temp::from_f(85.) <= Temp::from_f(87.), true);
//     }

//     #[test]
//     fn should_be_gt() {
//         assert_eq!(Temp::from_f(86.) > Temp::from_c(0.), true);
//         assert_eq!(Temp::from_f(86.) > Temp::from_c(100.), false);
//     }
// }
