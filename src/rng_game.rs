use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
  let answer = rand::thread_rng().gen_range(1..=100);

  println!(
    "Guess the number between 1 and 100 (psst... the answer is {})",
    answer
  );

  loop {
    println!("Input guess:");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&answer) {
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too Big"),
      Ordering::Equal => {
        println!("Correct!");
        break;
      }
    }
  }
}
