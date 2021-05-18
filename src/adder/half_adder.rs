use crate::nand as Nand;
use crate::adder::AdderResult;

fn half_adder(a: bool, b: bool) -> AdderResult {
    let sum = Nand::xor(a, b);
    let carry = Nand::and(a, b);
    return AdderResult { sum, carry };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_adder() {
        assert_eq!(
            half_adder(false, false),
            AdderResult {
                sum: false,
                carry: false
            }
        );

        assert_eq!(
            half_adder(false, true),
            AdderResult {
                sum: true,
                carry: false
            }
        );

        assert_eq!(
            half_adder(true, false),
            AdderResult {
                sum: true,
                carry: false
            }
        );

        assert_eq!(
            half_adder(true, true),
            AdderResult {
                sum: false,
                carry: true
            }
        );
    }
}
