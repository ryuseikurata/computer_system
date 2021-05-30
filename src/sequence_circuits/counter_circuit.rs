use crate::adders::incrementer16;
use crate::gates::{mux16, mux4way16};

use super::{register::Register, word::Word};

pub struct Counter {
    register: Register,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            register: Register::new(),
        }
    }
    /// 参考(https://nihemak.hatenablog.com/entry/2019/04/28/150541#PC)
    pub fn calc(&mut self, input: Word, inc: bool, load: bool, reset: bool) -> Word {
        mux4way16::calc(
            mux16::calc(
                self.register.out(),
                incrementer16::calc(self.register.out()),
                inc,
            ),
            input,
            [false; 16],
            [false; 16],
            [load, reset],
        )
    }
}
