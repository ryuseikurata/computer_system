use crate::sequence_circuits::register::Word;

use super::{mux16, mux4way16};

pub fn calc(
    a: Word,
    b: Word,
    c: Word,
    d: Word,
    e: Word,
    f: Word,
    g: Word,
    h: Word,
    sel: [bool; 3],
) -> Word {
    mux16::calc(
        mux4way16::calc(a, b, c, d, [sel[0], sel[1]]),
        mux4way16::calc(e, f, g, h, [sel[0], sel[1]]),
        sel[2],
    )
}

#[cfg(test)]
mod tests {
    use crate::const_values;

    use super::*;

    #[test]
    fn test() {
        let a = const_values::FULL;
        let b = const_values::ONE;
        let c = const_values::ZERO;
        let d = const_values::SAMPLE_1;
        let e = const_values::SAMPLE_2;
        let f = const_values::SAMPLE_3;
        let g = const_values::SAMPLE_4;
        let h = const_values::SAMPLE_5;
        assert_eq!(calc(a, b, c, d, e, f, g, h, [false, false, false]), a);
        assert_eq!(calc(a, b, c, d, e, f, g, h, [true, false, false]), b);
        assert_eq!(calc(a, b, c, d, e, f, g, h, [false, true, false]), c);
        assert_eq!(calc(a, b, c, d, e, f, g, h, [true, true, false]), d);
        assert_eq!(calc(a, b, c, d, e, f, g, h, [false, false, true]), e);
        assert_eq!(calc(a, b, c, d, e, f, g, h, [true, false, true]), f);
        assert_eq!(calc(a, b, c, d, e, f, g, h, [false, true, true]), g);
        assert_eq!(calc(a, b, c, d, e, f, g, h, [true, true, true]), h);
    }
}
