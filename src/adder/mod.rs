pub mod half_adder;
#[derive(PartialEq, Debug)]
pub struct AdderResult {
  pub sum: bool,
  pub carry: bool,
}
