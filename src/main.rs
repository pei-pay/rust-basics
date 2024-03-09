mod basic;
mod ownership;
mod structure;

fn main() {
  println!("===Basic===");
  basic::main();
  println!("===Ownership===");
  ownership::main();
  println!("===Struct===");
  structure::main();
}
