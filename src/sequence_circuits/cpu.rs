use super::{counter_circuit::Counter, register::Register, word::Word};

pub struct CPU {
    a_register: Register,
    b_register: Register,
    outM: Word,
    write_dist: Word,
    pc: Counter,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a_register: Register::new(),
            b_register: Register::new(),
            outM: [false; 16],
            write_dist: [false; 16],
            pc: Counter::new(),
        }
    }

    pub fn decode(instruction: Word) -> (bool, [bool; 2], bool, [bool; 6], [bool; 3], [bool; 3]) {
        (
            instruction[0],
            [instruction[1], instruction[2]],
            instruction[3],
            [
                instruction[4],
                instruction[5],
                instruction[6],
                instruction[7],
                instruction[8],
                instruction[9],
            ],
            [instruction[10], instruction[11], instruction[12]],
            [instruction[13], instruction[14], instruction[15]],
        )
    }
}
