// this is custom type KpS (SmartPointer due to Samadhi from y_traits [-5 to +5] in observable
//enforcements of its validity behaviours in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpS {
  value: i32,
}
impl KpS {
  pub fn new(value: i32) -> KpS {
    if value < 1 || value > 7 {
            panic!("KpS value must be between 1 to 7, got {}.", value);
    }

    KpS { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

