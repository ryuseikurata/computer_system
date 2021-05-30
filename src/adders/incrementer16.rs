use crate::sequence_circuits::word::Word;

use super::adder16;

pub fn calc(input: Word) -> Word {
    adder16::calc(
        input,
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, true,
        ],
    )
}
