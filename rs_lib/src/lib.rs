use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = paperIdToLevel)]
pub fn paper_id_to_level(id: &str) -> String {
    match id {
        "p1" | "p2" | "p3" | "p4" => "foundation".to_string(),
        "p5" | "p6" | "p7" | "p8" | "p9" | "p10" | "p11" | "p12" => "intermediate".to_string(),
        "p13" | "p14" | "p15" | "p16" | "p17" | "p18" | "p19" | "p20" | "p20A" | "p20B" | "p20C" => "final".to_string(),
        _ => "unknown".to_string(),
    }
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[wasm_bindgen]
pub struct Greeter {
  name: String,
}

#[wasm_bindgen]
impl Greeter {
  #[wasm_bindgen(constructor)]
  pub fn new(name: String) -> Self {
    Self { name }
  }

  pub fn greet(&self) -> String {
    format!("Hello {}!", self.name)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_adds() {
    let result = add(1, 2);
    assert_eq!(result, 3);
  }

  #[test]
  fn it_greets() {
    let greeter = Greeter::new("world".into());
    assert_eq!(greeter.greet(), "Hello world!");
  }
}
