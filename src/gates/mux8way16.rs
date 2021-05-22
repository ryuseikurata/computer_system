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
