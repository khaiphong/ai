#[derive(Debug)] // IBM
pub struct Granite {}

#[derive(Debug)] // meta
pub struct LLama {}

#[derive(Debug)] // xAI
pub struct Grok {}

// this is custom type KpI from traits [-5 to +5] in observable enforcements of its validity
// and behaviours in using Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpI {
  value: i32,
}
impl KpI {
  pub fn new(value: i32) -> KpI {
    if value < -5 || value > 5 {
            panic!("KpI value must be between -5 to +5, got {}.", value);
    }

    KpI { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

