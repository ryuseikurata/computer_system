use crate::sequence_circuits::register::Word;

use super::not;

pub fn calc(a: Word) -> Word {
    [
        not::calc(a[0]),
        not::calc(a[1]),
        not::calc(a[2]),
        not::calc(a[3]),
        not::calc(a[4]),
        not::calc(a[5]),
        not::calc(a[6]),
        not::calc(a[7]),
        not::calc(a[8]),
        not::calc(a[9]),
        not::calc(a[10]),
        not::calc(a[11]),
        not::calc(a[12]),
        not::calc(a[13]),
        not::calc(a[14]),
        not::calc(a[15]),
    ]
}
