#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn new() -> Rectangle {
    Rectangle {
      width: 10,
      height: 10,
    }
  }

  fn get_area(&self) -> u32 {
    &self.width * &self.height
  }
}

fn main() {
  let r = Rectangle::new();
  let area = r.get_area();

  println!("our struct: {:#?}", r);
  println!("area: {}", area);
}
