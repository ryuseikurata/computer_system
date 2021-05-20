use crate::adders::AdderResult;
use crate::gates;
pub struct FullAdder {
    a: bool,
    b: bool,
}

impl FullAdder {
    fn calc(a: bool, b: bool) -> AdderResult {
        let sum = gates::xor::calc(a, b);
        let carry = gates::and::calc(a, b);
        AdderResult { sum, carry }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_adder() {
        assert_eq!(
            FullAdder::calc(false, false),
            AdderResult {
                sum: false,
                carry: false
            }
        );

        assert_eq!(
            FullAdder::calc(false, true),
            AdderResult {
                sum: true,
                carry: false
            }
        );

        assert_eq!(
            FullAdder::calc(true, false),
            AdderResult {
                sum: true,
                carry: false
            }
        );

        assert_eq!(
            FullAdder::calc(true, true),
            AdderResult {
                sum: false,
                carry: true
            }
        );
    }
}
