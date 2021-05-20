use crate::gates;

pub fn calc(a: bool, b: bool) -> bool {
    let temp = gates::nand::calc(a, b);
    gates::nand::calc(gates::nand::calc(a, temp), gates::nand::calc(b, temp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        assert_eq!(calc(true, true), false);
        assert_eq!(calc(true, false), true);
        assert_eq!(calc(false, false), false);
        assert_eq!(calc(false, true), true);
    }
}
