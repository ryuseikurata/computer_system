use crate::gates;
pub fn calc(a: bool) -> bool {
    gates::nand::calc(a, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not() {
        assert_eq!(calc(false), true);
        assert_eq!(calc(true), false);
    }
}
