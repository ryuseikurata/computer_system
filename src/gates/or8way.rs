use crate::sequence_circuits::register::Word;

use super::or;

pub fn calc(input: [bool; 8]) -> bool {
    let w0 = or::calc(input[0], input[1]);
    let w1 = or::calc(input[2], input[3]);
    let w2 = or::calc(input[4], input[5]);
    let w3 = or::calc(input[6], input[7]);
    let w4 = or::calc(w0, w1);
    let w5 = or::calc(w2, w3);
    or::calc(w4, w5)
}
