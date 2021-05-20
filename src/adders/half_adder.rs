use crate::adders::AdderResult;
use crate::gates;

pub struct HalfAdder {
    a: bool,
    b: bool,
}

impl HalfAdder {
    pub fn calc(a: bool, b: bool) -> AdderResult {
        let sum = gates::xor::calc(a, b);
        let carry = gates::and::calc(a, b);
        AdderResult { sum, carry }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_adder() {
        assert_eq!(
            HalfAdder::calc(false, false),
            AdderResult {
                carry: false,
                sum: false,
            }
        );

        assert_eq!(
            HalfAdder::calc(false, true),
            AdderResult {
                carry: false,
                sum: true,
            }
        );

        assert_eq!(
            HalfAdder::calc(true, false),
            AdderResult {
                carry: false,
                sum: true,
            }
        );

        assert_eq!(
            HalfAdder::calc(true, true),
            AdderResult {
                carry: true,
                sum: false,
            }
        );
    }
}
