mod basic;
mod enum_type;
mod ownership;
mod structure;

fn main() {
  println!("===Basic===");
  basic::main();
  println!("===Ownership===");
  ownership::main();
  println!("===Struct===");
  structure::main();
  println!("===Enum===");
  enum_type::main()
}
