use crate::{
    const_values,
    gates::{and16, mux16, not, not16, or, or8way},
    sequence_circuits::register::Word,
};

use super::adder16;
/// x[16], y[16], 二つの16ビットデータ入力\
/// zx 入力xをゼロにする\
/// nx 入力xを反転(negate)する\
/// zy 入力yをゼロにする\
/// ny 入力yを反転(negate)する\
/// f 関数コード: trueは加算、falseはAnd演算\
/// no 出力(out)を反転(negate)

pub fn calc(
    x: Word,
    y: Word,
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> (Word, bool, bool) {
    let x = zero_or_negate(x, zx, nx);

    let y = zero_or_negate(y, zy, ny);

    let f_result = add_or_and(x, y, f);

    let no_result = mux16::calc(f_result, not16::calc(f_result), no);

    let zr = equal_zero(no_result);

    let ng = no_result[15];
    (no_result, zr, ng)
}

fn zero_or_negate(input: Word, z: bool, n: bool) -> Word {
    let zinput = mux16::calc(input, const_values::ZERO, z);
    mux16::calc(zinput, not16::calc(zinput), n)
}

fn add_or_and(a: Word, b: Word, f: bool) -> Word {
    mux16::calc(and16::calc(a, b), adder16::calc(a, b), f)
}

fn equal_zero(input: Word) -> bool {
    let c = or::calc(
        or8way::calc([
            input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7],
        ]),
        or8way::calc([
            input[8], input[9], input[10], input[11], input[12], input[13], input[14], input[15],
        ]),
    );
    not::calc(c)
}
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let x = const_values::SAMPLE_1;
        let y = const_values::SAMPLE_2;

        assert_eq!(
            calc(x, y, true, false, true, false, true, false),
            (const_values::ZERO, true, false)
        );
        assert_eq!(
            calc(x, y, true, true, true, true, true, true),
            (const_values::ONE, false, false)
        );
        assert_eq!(
            calc(x, y, true, true, true, false, true, false),
            (const_values::FULL, false, true)
        );
        assert_eq!(
            calc(x, y, false, false, true, true, false, false),
            (x, false, false)
        );

        assert_eq!(
            calc(x, y, false, false, true, true, false, true),
            (not16::calc(x), false, true)
        );

        assert_eq!(
            calc(x, y, true, true, false, false, false, false),
            (y, false, false)
        );

        assert_eq!(
            calc(x, y, true, true, false, false, false, true),
            (not16::calc(y), false, true)
        )
    }

    #[test]
    fn test_add_or_and() {
        let x = const_values::FULL;
        let y = const_values::SAMPLE_2;
        assert_eq!(add_or_and(x, y, false), y)
    }
}

/*
let cy = y
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .concat();
    let c = f_result
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .concat();
    println!("{}", c);
*/
