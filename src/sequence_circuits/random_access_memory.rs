use crate::gates::mux8way16;

use super::register::{Register, Word};

pub struct RAM8 {
    registers: [Register; 8],
}

impl RAM8 {
    pub fn new() -> Self {
        Self {
            registers: [Register::new(); 8],
        }
    }

    /// registerは8個なので、3ビットのアドレスでどの位置かを指定することができる
    pub fn out(&self, address: [bool; 3]) -> Word {
        mux8way16::calc(
            self.registers[0].out(),
            self.registers[1].out(),
            self.registers[2].out(),
            self.registers[3].out(),
            self.registers[4].out(),
            self.registers[5].out(),
            self.registers[6].out(),
            self.registers[7].out(),
            address,
        )
    }

    pub fn clock(&mut self, address: [bool; 3], input: Word, load: bool) -> Word {
        [true; 16]
    }
}
