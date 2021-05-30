use crate::sequence_circuits::word::Word;

use super::mux;

/// trueだったら右側\
/// falseだったら左側
pub fn calc(a: Word, b: Word, sel: bool) -> Word {
    [
        mux::calc(a[0], b[0], sel),
        mux::calc(a[1], b[1], sel),
        mux::calc(a[2], b[2], sel),
        mux::calc(a[3], b[3], sel),
        mux::calc(a[4], b[4], sel),
        mux::calc(a[5], b[5], sel),
        mux::calc(a[6], b[6], sel),
        mux::calc(a[7], b[7], sel),
        mux::calc(a[8], b[8], sel),
        mux::calc(a[9], b[9], sel),
        mux::calc(a[10], b[10], sel),
        mux::calc(a[11], b[11], sel),
        mux::calc(a[12], b[12], sel),
        mux::calc(a[13], b[13], sel),
        mux::calc(a[14], b[14], sel),
        mux::calc(a[15], b[15], sel),
    ]
}
