pub fn calc(a: bool, b: bool, sel: bool) -> bool {
  if sel == false {
    a
  } else {
    b
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_mux() {
    assert_eq!(calc(true, false, true), false)
  }
}
