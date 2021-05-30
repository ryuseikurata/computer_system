use crate::gates::mux;

pub struct Clock {
    state: bool,
}

impl Clock {
    pub fn state(&self) -> bool {
        self.state
    }

    /// stateがfalse(tok)であればtrue(tick)
    pub fn next(&mut self) {
        self.state = mux::calc(true, false, self.state)
    }

    pub fn new() -> Self {
        Self { state: false }
    }
}
