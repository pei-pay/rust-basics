mod if_let;
mod intro;
mod match_control;

pub fn main() {
  println!("==intro==");
  intro::main();
  println!("==match==");
  match_control::main();
  println!("==if let==");
  if_let::main();
}
