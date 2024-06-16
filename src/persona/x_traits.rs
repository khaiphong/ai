// this is custom type KpI from x_traits [-5 to +5] in observable enforcements of its validity
// and behaviours in using Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpI {
  value: i32,
}
impl KpI {
  pub fn new(value: i32) -> KpI {
    if value < -4 || value > 4 {
            panic!("KpI value must be between -4 to +4, got {}.", value);
    }

    KpI { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

