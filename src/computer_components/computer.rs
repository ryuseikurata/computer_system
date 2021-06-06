use crate::sequence_circuits::{cpu::CPU, read_only_memory::ROM32K};

use super::{keyboard::Keyboard, screen::Screen};

pub struct Computer {
    rom: ROM32K,
    cpu: CPU,
    // memory: Memory<S, K>,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            rom: ROM32K::new(),
            cpu: CPU::new(),
        }
    }

    pub  fn clock(&mut self) {
      self.cpu.cycle(in_memory, instruction, reset)
    }
}

/*
impl<S: Screen, K: Keyboard> Computer<S, K> {
    pub fn new() -> Self {
        Self {
            rom: ROM32K::new(),
            cpu: CPU::new(),
            // memory: Memory::new(),
        }
    }

    pub fn tick(&mut self, reset: bool) {
        let (memory, write_to_memory, address, pc) = self.cpu.out();
    }
}
*/
