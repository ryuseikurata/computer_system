use crate::sequence_circuits::register::Word;

pub const ZERO: Word = [false; 16];

pub const ONE: Word = [
    true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false,
];
pub const FULL: Word = [true; 16];

pub const SAMPLE_1: Word = [
    true, false, true, false, true, true, false, true, false, true, false, true, false, true,
    false, true,
];

pub const SAMPLE_2: Word = [
    false, true, false, true, false, true, false, true, false, true, false, true, false, true,
    false, true,
];

pub const SAMPLE_3: Word = [
    false, true, false, true, true, true, false, true, false, true, true, true, false, true, false,
    false,
];

pub const SAMPLE_4: Word = [
    false, true, true, true, true, true, true, true, false, true, true, true, false, false, false,
    false,
];

pub const SAMPLE_5: Word = [
    false, true, true, false, true, false, false, true, false, true, false, true, false, true,
    false, false,
];

pub const SAMPLE_6: Word = [
    false, true, true, false, true, true, true, true, true, true, true, true, false, true,
    false, false,
];
