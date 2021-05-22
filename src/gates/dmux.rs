use super::{and, not};

pub fn calc(input: bool, sel: bool) -> [bool; 2] {
    let a = and::calc(input, not::calc(sel));
    let b = and::calc(input, sel);
    [a, b]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmux() {
        assert_eq!(calc(true, false), [true, false]);
        assert_eq!(calc(true, true), [false, true]);
        assert_eq!(calc(false, false), [false, false]);
        assert_eq!(calc(false, true), [false, false]);
    }
}
