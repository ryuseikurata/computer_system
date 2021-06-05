use super::{keyboard::Keyboard, screen::Screen};
use crate::sequence_circuits::random_access_memory::RAM16K;

pub struct Memory<S: Screen, K: Keyboard> {
    address: [bool; 15],
    ram: RAM16K,
    screen: S,
    keyboard: K,
}

impl<S: Screen, K: Keyboard> Memory<S, K> {
    pub fn new() -> Self {
        Self {
            address: [false; 15],
            ram: RAM16K::new(),
            screen: Screen::new(),
            keyboard: Keyboard::new(),
        }
    }
}
