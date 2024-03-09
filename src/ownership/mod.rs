mod function;
mod references_and_borrowing;
mod slice;
mod string;

pub fn main() {
  println!("==string==");
  string::main();
  println!("==function==");
  function::main();
  println!("==references and borrowing==");
  references_and_borrowing::main();
  println!("===slice==");
  slice::main();
}
