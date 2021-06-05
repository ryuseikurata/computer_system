#[derive(Copy, Clone)]
pub struct FlipFlop {
    bit: bool,
}

impl FlipFlop {
    pub fn new() -> FlipFlop {
        FlipFlop { bit: false }
    }

    pub fn out(&self) -> bool {
        self.bit
    }

    pub fn clock(&mut self, a: bool) {
        self.bit = a
    }
}
