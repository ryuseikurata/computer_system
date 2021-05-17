/// 0: false
/// 1: true
/// 基礎の関数
/// これを使用して他の論理関数を作成する
pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

fn not(input: bool) -> bool {
    nand(input, input)
}

fn and(a: bool, b: bool) -> bool {
    !nand(a, b)
}

fn or(a: bool, b: bool) -> bool {
    nand(!a, !b)
}

fn xor(a: bool, b: bool) -> bool {
    nand(a, b)
}

fn mux(a: bool, b: bool, sel: bool) -> bool {
    if sel == false {
        a
    } else {
        b
    }
}

fn dmux(input: bool, sel: bool) -> (bool, bool) {
    if sel == false {
        (input, false)
    } else {
        (false, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nand() {
        assert_eq!(nand(true, true), false);
    }

    #[test]
    fn test_and() {
        assert_eq!(or(false, false), false);
    }

    #[test]
    fn test_not() {
        assert_eq!(not(false), true);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(true, true), false)
    }

    #[test]
    fn test_mux() {
        assert_eq!(mux(true, false, true), false)
    }

    #[test]
    fn test_dmux() {
        assert_eq!(dmux(true, false), (true, false))
    }
}
