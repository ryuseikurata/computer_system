use crate::sequence_circuits::{random_access_memory::RAM8K, word::Word};

pub trait Screen {
    fn new() -> Self;
    fn out(&self, address: [bool; 13]) -> Word;
    fn clock(&mut self, address: [bool; 13], input: Word, load: bool);
}

pub struct DummyScreen {
    ram: RAM8K,
}

impl Screen for DummyScreen {
    fn new() -> Self {
        Self { ram: RAM8K::new() }
    }

    fn out(&self, address: [bool; 13]) -> Word {
        self.ram.out(address)
    }

    fn clock(&mut self, address: [bool; 13], input: Word, load: bool) {
        self.ram.clock(address, input, load);
    }
}
