
use crate::adder::AdderResult;
use crate::gates;

pub struct HalfAdder {
    a: bool,
    b: bool,
}

impl HalfAdder {
    fn new(a: bool, b: bool) -> HalfAdder {
       HalfAdder { a, b }
    }

    fn calc(&self) -> AdderResult {
      let sum = gates::xor::calc(self.a, self.b);
      let carry = gates::and::calc(self.a, self.b);
      AdderResult {sum, carry}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_adder() {
        assert_eq!(
          HalfAdder::new(false, false).calc(),
            AdderResult {
                sum: false,
                carry: false
            }
        );

        assert_eq!(
          HalfAdder::new(false, true).calc(),
            AdderResult {
                sum: true,
                carry: false
            }
        );

        assert_eq!(
          HalfAdder::new(true, false).calc(),
            AdderResult {
                sum: true,
                carry: false
            }
        );

        assert_eq!(
          HalfAdder::new(true, true).calc(),
            AdderResult {
                sum: false,
                carry: true
            }
        );
    }
}
