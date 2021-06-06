use crate::{
    adders::{full_adder, half_adder},
    sequence_circuits::word::Word,
};

pub fn calc(a: Word, b: Word) -> Word {
    let result0 = half_adder::HalfAdder::calc(a[0], b[0]);
    let result1 = full_adder::FullAdder::calc(a[1], b[1], result0.carry);
    let result2 = full_adder::FullAdder::calc(a[2], b[2], result1.carry);
    let result3 = full_adder::FullAdder::calc(a[3], b[3], result2.carry);
    let result4 = full_adder::FullAdder::calc(a[4], b[4], result3.carry);
    let result5 = full_adder::FullAdder::calc(a[5], b[5], result4.carry);
    let result6 = full_adder::FullAdder::calc(a[6], b[6], result5.carry);
    let result7 = full_adder::FullAdder::calc(a[7], b[7], result6.carry);
    let result8 = full_adder::FullAdder::calc(a[8], b[8], result7.carry);
    let result9 = full_adder::FullAdder::calc(a[9], b[9], result8.carry);
    let result10 = full_adder::FullAdder::calc(a[10], b[10], result9.carry);
    let result11 = full_adder::FullAdder::calc(a[11], b[11], result10.carry);
    let result12 = full_adder::FullAdder::calc(a[12], b[12], result11.carry);
    let result13 = full_adder::FullAdder::calc(a[13], b[13], result12.carry);
    let result14 = full_adder::FullAdder::calc(a[14], b[14], result13.carry);
    let result15 = full_adder::FullAdder::calc(a[15], b[15], result14.carry);
    [
        result0.sum,
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
    ]
}

#[cfg(test)]
mod test {
    use crate::{adders::adder16::calc, const_values};

    #[test]
    fn one_plus_one() {
        let one = const_values::ONE;
        let out = calc(one, one);
        assert_eq!(out, const_values::TWO);
    }
}
