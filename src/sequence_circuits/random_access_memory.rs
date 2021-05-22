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
      
    }

    pub fn clock()
}
