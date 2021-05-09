// 0: false
// 1: true
pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

fn not(input: bool) -> bool {
    nand(input, input)
}

fn and(a: bool, b:bool)-> bool {
    !nand(a, b)
}

fn or(a: bool, b:bool) -> bool {
    nand(!a, !b)
}

fn xor(a: bool, b: bool) -> bool {
    nand(a, b)
}

fn mux(a: i8, b:i8, sel: i8) -> i8 {
    if sel == 0 {
        a
    } else {
        b
    }
}

fn dmux(input: i8, sel: i8) -> (i8,i8) {
    if sel == 0 {
        (input, 0)
    } else {
        (0, input)
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
    fn test_xor() {
        assert_eq!(xor(a, b))
    }
}
