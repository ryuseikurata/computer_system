use crate::adders::{half_adder, AdderResult};
use crate::gates::or;
pub struct FullAdder {
    a: bool,
    b: bool,
}

impl FullAdder {
    pub fn calc(a: bool, b: bool, c: bool) -> AdderResult {
        let half_adder_result_1 = half_adder::HalfAdder::calc(a, b);

        let half_adder_result_2 = half_adder::HalfAdder::calc(half_adder_result_1.sum, c);

        AdderResult {
            carry: or::calc(half_adder_result_2.carry, half_adder_result_1.carry),
            sum: half_adder_result_2.sum,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_adder() {
        assert_eq!(
            FullAdder::calc(false, false, false),
            AdderResult {
                carry: false,
                sum: false,
            }
        );
        assert_eq!(
            FullAdder::calc(false, false, true),
            AdderResult {
                carry: false,
                sum: true,
            }
        );

        assert_eq!(
            FullAdder::calc(false, true, false),
            AdderResult {
                carry: false,
                sum: true,
            }
        );

        assert_eq!(
            FullAdder::calc(false, true, true),
            AdderResult {
                carry: true,
                sum: false,
            }
        );

        assert_eq!(
            FullAdder::calc(true, false, false),
            AdderResult {
                carry: false,
                sum: true,
            }
        );

        assert_eq!(
            FullAdder::calc(true, false, true),
            AdderResult {
                carry: true,
                sum: false,
            }
        );

        assert_eq!(
            FullAdder::calc(true, true, false),
            AdderResult {
                carry: true,
                sum: false,
            }
        );

        assert_eq!(
            FullAdder::calc(true, true, true),
            AdderResult {
                carry: true,
                sum: true,
            }
        );
    }
}
