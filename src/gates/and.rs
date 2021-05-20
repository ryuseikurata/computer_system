use crate::gates;
pub fn calc(a: bool, b: bool) -> bool {
    gates::nand::calc(gates::not::calc(a), gates::not::calc(b))
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(calc(true, true), true);
        assert_eq!(calc(true, false), false);
        assert_eq!(calc(false, true), false);
        assert_eq!(calc(false, false), false);
    }
}
