use crate::sequence_circuits::{
    counter_circuit::Counter, cpu::CPU, read_only_memory::ROM32K, word::Word,
};

use super::{keyboard::Keyboard, memory_map::Memory, screen::Screen};

pub struct Computer<S: Screen, K: Keyboard> {
    rom: ROM32K,
    cpu: CPU,
    memory: Memory<S, K>,
}

impl<S: Screen, K: Keyboard> Computer<S, K> {
    pub fn new() -> Self {
        Self {
            rom: ROM32K::new(),
            cpu: CPU::new(),
            memory: Memory::new(),
        }
    }

    pub fn clock(&mut self, reset: bool) {
        let (cpu_output, write_to_memory, address, pc) = self.cpu.out();
        self.memory.clock(address, write_to_memory, cpu_output);
        let memory_data = self.memory.out(address);
        // ROMにPCの値の書き込みが必要かも？
        let instruction = self.rom.out();
        self.cpu.cycle(memory_data, instruction, reset);
    }

    pub fn a(&self) -> Word {
        self.cpu.a()
    }
    pub fn d(&self) -> Word {
        self.cpu.d()
    }
    pub fn m(&self, address: [bool; 15]) -> Word {
        self.memory.out(address)
    }

    pub fn pc(&self) -> &Counter {
        self.cpu.pc()
    }
    pub fn screen(&self) -> &S {
        self.memory.screen()
    }
}
