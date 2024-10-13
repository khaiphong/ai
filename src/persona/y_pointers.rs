// this is custom type KpP (SmartPointer due to Samadhi from y_pointers [-5 to +5] in 
//observable enforcements of its behaviours in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpP {
  value: i32,
}
impl KpP {
  pub fn new(value: i32) -> KpP {
    if value < 1 || value > 7 {
            panic!("KpP value must be between 1 to 7, got {}.", value);
    }

    KpP { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

