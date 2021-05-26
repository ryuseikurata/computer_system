use crate::sequence_circuits::register::Word;

use super::mux16;

pub fn calc(a: Word, b: Word, c: Word, d: Word, sel: [bool; 2]) -> Word {
    mux16::calc(mux16::calc(a, b, sel[0]), mux16::calc(c, d, sel[0]), sel[1])
}

#[cfg(test)]
mod tests {
    use crate::const_values;

    use super::*;

    #[test]
    fn test() {
        let a = [
            true, false, true, false, true, true, false, true, false, true, false, true, false,
            true, false, true,
        ];
        let b = [
            false, true, false, true, false, true, false, true, false, true, false, true, false,
            true, false, true,
        ];
        let c = const_values::FULL;
        let d = const_values::ZERO;
        assert_eq!(calc(a, b, c, d, [false, false]), a);
        assert_eq!(calc(a, b, c, d, [true, false]), b);
        assert_eq!(calc(a, b, c, d, [false, true]), c);
        assert_eq!(calc(a, b, c, d, [true, true]), d);
    }
}
