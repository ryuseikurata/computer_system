use super::{dmux, dmux4way};

pub fn calc(input: bool, sel: [bool; 3]) -> [bool; 8] {
    let [abcd, efgh] = dmux::calc(input, sel[2]);
    let [a, b, c, d] = dmux4way::calc(abcd, [sel[0], sel[1]]);
    let [e, f, g, h] = dmux4way::calc(efgh, [sel[0], sel[1]]);
    [a, b, c, d, e, f, g, h]
}
