use std::io;

fn main() {
  println!("Guess the number!");
  println!("please input your guess.");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("fail to read line");

  println!("your guess is {}", guess);
}
