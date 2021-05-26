pub mod half_adder;
pub mod full_adder;
pub mod adder16;
pub mod incrementer16;
pub mod alu;
#[derive(PartialEq, Debug)]
pub struct AdderResult {
  pub sum: bool,
  pub carry: bool,
}
