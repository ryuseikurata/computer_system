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
        let input1 = true;
        assert_eq!(calc(input1, false), [input1, false]);
        assert_eq!(calc(input1, true), [false, input1]);

        let input2 = false;
        assert_eq!(calc(input2, false), [input2, false]);
        assert_eq!(calc(input2, true), [false, input2]);
    }
}
