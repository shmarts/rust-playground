#[derive(Debug)]
enum Size {
  S,
  M,
  L,
}

impl Size {
  fn convert(&self) -> u8 {
    match &self {
      Size::S => 10,
      Size::M => 20,
      Size::L => 30,
    }
  }
}

fn add_one(n: Option<i32>) -> i32 {
  match n {
    None => 1,
    Some(i) => i + 1,
  }
}

fn match_u8(n: u8) -> String {
  match n {
    10 => String::from("ten!"),
    _ => String::from("not matched ):"),
  }
}

fn check_if_ten(n: Option<u8>) {
  if let Some(10) = n {
    println!("ten again!")
  }
}

fn main() {
  println!(
    "{}, {}, {}",
    Size::S.convert(),
    Size::M.convert(),
    Size::L.convert()
  );
  println!("{:#?}", add_one(Some(9)));
  println!("{}", match_u8(10));
  check_if_ten(Some(10));
}
