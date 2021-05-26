use crate::sequence_circuits::register::Word;

use super::and;

pub fn calc(a: Word, b: Word) -> Word {
    [
        and::calc(a[0], b[0]),
        and::calc(a[1], b[1]),
        and::calc(a[2], b[2]),
        and::calc(a[3], b[3]),
        and::calc(a[4], b[4]),
        and::calc(a[5], b[5]),
        and::calc(a[6], b[6]),
        and::calc(a[7], b[7]),
        and::calc(a[8], b[8]),
        and::calc(a[9], b[9]),
        and::calc(a[10], b[10]),
        and::calc(a[11], b[11]),
        and::calc(a[12], b[12]),
        and::calc(a[13], b[13]),
        and::calc(a[14], b[14]),
        and::calc(a[15], b[15]),
    ]
}
