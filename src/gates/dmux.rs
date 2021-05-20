pub fn calc(input: bool, sel: bool) -> (bool, bool) {
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
  fn test_dmux()  {
    assert_eq!(calc(true, false), (true, false))
  }
}
