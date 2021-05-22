use super::{and, not, or};

pub fn calc(a: bool, b: bool, sel: bool) -> bool {
    let result1 = and::calc(a, not::calc(sel));
    let result2 = and::calc(b, sel);
    or::calc(result1, result2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mux() {
        assert_eq!(calc(false, false, false), false);
        assert_eq!(calc(false, false, true), false);
        assert_eq!(calc(true, false, true), false);
        assert_eq!(calc(true, false, false), true);
        assert_eq!(calc(false, true, true), true);
        assert_eq!(calc(false, true, false), false);
        assert_eq!(calc(true, true, true), true);
        assert_eq!(calc(true, true, false), true);
    }
}
