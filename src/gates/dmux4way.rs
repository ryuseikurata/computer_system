use super::dmux;

/// 一つがinputの値が入り、他はfalseが入るようになる関数\
/// テストコードで仕様を確認
pub fn calc(input: bool, sel: [bool; 2]) -> [bool; 4] {
    let [ab, cd] = dmux::calc(input, sel[1]);
    let [a, b] = dmux::calc(ab, sel[0]);
    let [c, d] = dmux::calc(cd, sel[0]);
    [a, b, c, d]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input1 = true;
        assert_eq!(calc(input1, [false, false]), [input1, false, false, false]);
        assert_eq!(calc(input1, [true, false]), [false, input1, false, false]);
        assert_eq!(calc(input1, [false, true]), [false, false, input1, false]);
        assert_eq!(calc(input1, [true, true]), [false, false, false, input1]);

        let input2 = false;
        assert_eq!(calc(input2, [false, false]), [input2, false, false, false]);
        assert_eq!(calc(input2, [true, false]), [false, input2, false, false]);
        assert_eq!(calc(input2, [false, true]), [false, false, input2, false]);
        assert_eq!(calc(input2, [true, true]), [false, false, false, input2]);
    }
}
