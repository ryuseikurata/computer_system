use super::flip_flop::FlipFlop;
use crate::gates::mux;
pub struct BitRegister {
    flipflop: FlipFlop,
}

impl BitRegister {
    pub fn new() -> Self {
        Self {
            flipflop: FlipFlop::new(),
        }
    }

    pub fn out(&self) -> bool {
        self.flipflop.out()
    }

    pub fn clock(&mut self, input: bool, load: bool) {
        self.flipflop.clock(mux::calc(self.out(), input, load))
    }
}
