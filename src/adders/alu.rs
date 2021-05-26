use crate::{
    const_values,
    gates::{and16, mux16, mux4way16, not16},
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

pub fn calc(x: Word, y: Word, zx: bool, nx: bool, zy: bool, ny: bool, f: bool, no: bool) -> Word {
    let x = mux4way16::calc(
        x,
        not16::calc(x),
        const_values::ZERO,
        const_values::FULL,
        [zx, nx],
    );
    let y = mux4way16::calc(
        y,
        not16::calc(y),
        const_values::ZERO,
        const_values::FULL,
        [zy, ny],
    );
    let result = mux4way16::calc(
        and16::calc(x, y),
        adder16::calc(x, y),
        not16::calc(and16::calc(x, y)),
        not16::calc(adder16::calc(x, y)),
        [no, f],
    );
    result
}

fn zero_or_negate(input: Word, z: bool, n: bool) -> Word {
    let zinput = mux16::calc(input, const_values::ZERO, z);
    mux16::calc(zinput, not16::calc(zinput), n)
}

fn add_or_and(a: Word, b: Word, f: bool) -> Word {
    mux16::calc(and16::calc(a, b), adder16::calc(a, b), f)
}
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = [
            true, false, true, false, true, false, true, false, true, false, true, false, true,
            false, true, false,
        ];
        let y = [
            false, true, false, true, false, true, true, false, true, false, true, false, true,
            false, true, false,
        ];

        assert_eq!(
            calc(x, y, true, false, true, false, true, false),
            const_values::ZERO
        );
        assert_eq!(
            calc(x, y, true, true, true, true, true, true),
            const_values::ONE
        );
        assert_eq!(
            calc(x, y, true, true, true, false, true, false),
            const_values::FULL
        );
        assert_eq!(calc(x, y, false, false, true, true, false, false), x);
        assert_eq!(
            calc(x, y, false, false, true, true, false, true),
            not16::calc(x)
        );
    }
}
