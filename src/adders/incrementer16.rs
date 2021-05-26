use super::adder16;
use crate::sequence_circuits::register::Word;

pub fn calc(input: Word) -> Word {
    adder16::calc(
        input,
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, true,
        ],
    )
}
