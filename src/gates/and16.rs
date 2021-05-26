use crate::sequence_circuits::register::Word;

use super::and;

pub fn calc(a: Word, b: Word) -> Word {
    [
        and::calc(a[0], b[0]),
        and::calc(a[1], b[1]),
        and::calc(a[2], a[2]),
        and::calc(a[3], a[3]),
        and::calc(a[4], a[4]),
        and::calc(a[5], a[5]),
        and::calc(a[6], a[6]),
        and::calc(a[7], a[7]),
        and::calc(a[8], a[8]),
        and::calc(a[9], a[9]),
        and::calc(a[10], a[10]),
        and::calc(a[11], a[11]),
        and::calc(a[12], a[12]),
        and::calc(a[13], a[13]),
        and::calc(a[14], a[14]),
        and::calc(a[15], a[15]),
    ]
}
