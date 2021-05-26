use crate::{
    adders::{full_adder, half_adder, AdderResult},
    sequence_circuits::register::Word,
};

pub fn calc(a: Word, b: Word) -> Word {
    let result1 = half_adder::HalfAdder::calc(a[0], b[0]);
    let result2 = full_adder::FullAdder::calc(a[1], b[1], result1.carry);
    let result3 = full_adder::FullAdder::calc(a[2], b[2], result2.carry);
    let result4 = full_adder::FullAdder::calc(a[3], b[3], result3.carry);
    let result5 = full_adder::FullAdder::calc(a[4], b[4], result4.carry);
    let result6 = full_adder::FullAdder::calc(a[5], b[5], result5.carry);
    let result7 = full_adder::FullAdder::calc(a[6], b[6], result6.carry);
    let result8 = full_adder::FullAdder::calc(a[7], b[7], result7.carry);
    let result9 = full_adder::FullAdder::calc(a[8], b[8], result8.carry);
    let result10 = full_adder::FullAdder::calc(a[9], b[9], result9.carry);
    let result11 = full_adder::FullAdder::calc(a[10], b[10], result10.carry);
    let result12 = full_adder::FullAdder::calc(a[11], b[11], result11.carry);
    let result13 = full_adder::FullAdder::calc(a[12], b[12], result12.carry);
    let result14 = full_adder::FullAdder::calc(a[13], b[13], result13.carry);
    let result15 = full_adder::FullAdder::calc(a[14], b[14], result14.carry);
    let result16 = full_adder::FullAdder::calc(a[15], b[15], result15.carry);
    [
        result1.sum,
        result2.sum,
        result3.sum,
        result4.sum,
        result5.sum,
        result6.sum,
        result7.sum,
        result8.sum,
        result9.sum,
        result10.sum,
        result11.sum,
        result12.sum,
        result13.sum,
        result14.sum,
        result15.sum,
        result16.sum,
    ]
}
