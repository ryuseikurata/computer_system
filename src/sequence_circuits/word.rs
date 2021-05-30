pub type Word = [bool; 16];

pub fn from_string(mut s: String) -> Word {
    s = s.split_terminator(' ').collect();
    let mut instruction = [false; 16];
    let mut i = 0usize;
    for bytes in s.bytes() {
        instruction[i] = match bytes {
            48 => false,
            49 => true,
            _ => panic!("NO"),
        };

        i = if i == 16 {
            panic!("`Word::from_string` fail: need less than 17.")
        } else {
            i + 1
        };
    }
    if i == 16 {
        instruction
    } else {
        panic!("`Word::from_string` fail: need more than 15")
    }
}

pub fn from_str(mut s: &str) -> Word {
    let s: String = s.split_terminator(' ').collect();
    let mut instruction = [false; 16];
    let mut i = 0usize;
    for bytes in s.bytes() {
        instruction[i] = match bytes {
            48 => false,
            49 => true,
            _ => panic!("NO"),
        };

        i = if i == 16 {
            panic!("`Word::from_string` fail: need less than 17.")
        } else {
            i + 1
        };
    }
    if i == 16 {
        instruction
    } else {
        panic!("`Word::from_string` fail: need more than 15")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::const_values;

    #[test]
    fn success_str() {
        assert_eq!(from_str("1111111111111111"), const_values::FULL);
    }
}
