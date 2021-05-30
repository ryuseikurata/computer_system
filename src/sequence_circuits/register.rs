use super::{bit_register::BitRegister, word::Word};

pub struct Register {
    bits: [BitRegister; 16],
}

impl Register {
    pub fn new() -> Self {
        Self {
            bits: [
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
                BitRegister::new(),
            ],
        }
    }
    /// 個別に初期化する実装が良さそうなのがない
    /// https://tyfkda.github.io/blog/2020/03/19/rust-init-array.html
    pub fn out(&self) -> Word {
        [
            self.bits[0].out(),
            self.bits[1].out(),
            self.bits[2].out(),
            self.bits[3].out(),
            self.bits[4].out(),
            self.bits[5].out(),
            self.bits[6].out(),
            self.bits[7].out(),
            self.bits[8].out(),
            self.bits[9].out(),
            self.bits[10].out(),
            self.bits[11].out(),
            self.bits[12].out(),
            self.bits[13].out(),
            self.bits[14].out(),
            self.bits[15].out(),
        ]
    }

    pub fn clock(&mut self, input: Word, load: bool) {
        for (i, &x) in input.iter().enumerate() {
            self.bits[i].clock(x, load)
        }
    }
}
