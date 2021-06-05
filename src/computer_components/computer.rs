use crate::sequence_circuits::{cpu::CPU, read_only_memory::ROM32K};

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
      memory: Memory:
    }
  }
}
