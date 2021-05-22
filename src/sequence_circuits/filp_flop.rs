pub struct FilpFlop {
    bit: bool,
}

impl FilpFlop {
    pub fn new(a: bool) -> FilpFlop {
        FlipFlop { a }
    }

    pub fn out(&self) -> FlipFlop {
        self
    }

    pub fn clock(&mut self, a: bool) -> FlipFlop {
        self.bit = a
    }
}
