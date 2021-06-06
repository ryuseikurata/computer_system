use crate::sequence_circuits::{register::Register, word::Word};

pub trait Keyboard {
    type State;
    fn new() -> Self;
    fn out(&self) -> Word;
    fn set_state(&mut self);
}

pub struct DummyKeyboard {
    register: Register,
}

impl Keyboard for DummyKeyboard {
    type State = ();
    fn new() -> Self {
        Self {
            register: Register::new(),
        }
    }

    fn out(&self) -> Word {
        self.register.out()
    }

    fn set_state(&mut self) {}
}
