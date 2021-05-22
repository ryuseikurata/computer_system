use crate::gates::{dmux8way, mux4way16, mux8way16};

use super::register::{Register, Word};

pub struct RAM8 {
    registers: [Register; 8],
}

impl RAM8 {
    pub fn new() -> Self {
        Self {
            registers: [
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
            ],
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

    pub fn clock(&mut self, address: [bool; 3], input: Word, load: bool) {
        let load = dmux8way::calc(load, address);
        self.registers[0].clock(input, load[0]);
        self.registers[1].clock(input, load[1]);
        self.registers[2].clock(input, load[2]);
        self.registers[3].clock(input, load[3]);
        self.registers[4].clock(input, load[4]);
        self.registers[5].clock(input, load[5]);
        self.registers[6].clock(input, load[6]);
        self.registers[7].clock(input, load[7]);
    }
}

pub struct RAM64 {
    rams: [RAM8; 8],
}

impl RAM64 {
    pub fn new() -> Self {
        Self {
            rams: [
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
            ],
        }
    }

    pub fn out(&self, address: [bool; 6]) -> Word {
        let lo = [address[0], address[1], address[2]];
        let hi = [address[3], address[4], address[5]];
        mux8way16::calc(
            self.rams[0].out(lo),
            self.rams[1].out(lo),
            self.rams[2].out(lo),
            self.rams[3].out(lo),
            self.rams[4].out(lo),
            self.rams[5].out(lo),
            self.rams[6].out(lo),
            self.rams[7].out(lo),
            hi,
        )
    }
}

pub struct RAM512 {
    rams: [RAM64; 8],
}

impl RAM512 {
    pub fn new() -> Self {
        Self {
            rams: [
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
            ],
        }
    }

    pub fn out(&self, address: [bool; 9]) -> Word {
        let lo = [
            address[0], address[1], address[2], address[3], address[4], address[5],
        ];
        let hi = [address[6], address[7], address[8]];
        mux8way16::calc(
            self.rams[0].out(lo),
            self.rams[1].out(lo),
            self.rams[2].out(lo),
            self.rams[3].out(lo),
            self.rams[4].out(lo),
            self.rams[5].out(lo),
            self.rams[6].out(lo),
            self.rams[7].out(lo),
            hi,
        )
    }
}

pub struct RAM4K {
    rams: [RAM512; 8],
}

impl RAM4K {
    pub fn new() -> Self {
        Self {
            rams: [
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
            ],
        }
    }

    pub fn out(&self, address: [bool; 12]) -> Word {
        let lo = [
            address[0], address[1], address[2], address[3], address[4], address[5], address[6],
            address[7], address[8],
        ];

        let hi = [address[9], address[10], address[11]];

        mux8way16::calc(
            self.rams[0].out(lo),
            self.rams[1].out(lo),
            self.rams[2].out(lo),
            self.rams[3].out(lo),
            self.rams[4].out(lo),
            self.rams[5].out(lo),
            self.rams[6].out(lo),
            self.rams[7].out(lo),
            hi,
        )
    }
}

pub struct RAM16K {
    rams: [RAM4K; 4],
}

impl RAM16K {
    pub fn new() -> Self {
        Self {
            rams: [RAM4K::new(), RAM4K::new(), RAM4K::new(), RAM4K::new()],
        }
    }

    pub fn out(&self, address: [bool; 14]) -> Word {
        let lo = [
            address[0],
            address[1],
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
        ];
        let hi = [address[12], address[13]];

        mux4way16::calc(
            self.rams[0].out(lo),
            self.rams[1].out(lo),
            self.rams[2].out(lo),
            self.rams[3].out(lo),
            hi,
        )
    }
}
