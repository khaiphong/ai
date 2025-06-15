// evaluate each p1_EmptyTheContent ranging from [0 to +4] from qualified to NoThingness
// in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpP1 {
  value: i32,
}
impl KpP1 {
  pub fn new(value: i32) -> KpP1 {
    if value < 0 || value > 4 {
            panic!("KpP1 value must be between 0 to 4, got {}.", value);
    }

    KpP1 { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

