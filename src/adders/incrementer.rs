use crate::adders::{half_adder, AdderResult};

pub struct Incrementer {
    a: bool,
    b: bool,
}

impl Incrementer {
    pub fn calc(input: AdderResult) -> AdderResult {
        half_adder::HalfAdder::calc(input.sum, true)
    }
}
