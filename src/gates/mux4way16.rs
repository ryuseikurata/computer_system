use crate::sequence_circuits::register::Word;

use super::mux16;

pub fn calc(a: Word, b: Word, c: Word, d: Word, sel: [bool; 2]) -> Word {
    mux16::calc(mux16::calc(a, b, sel[0]), mux16::calc(c, d, sel[0]), sel[1])
}
