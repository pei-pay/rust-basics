pub fn main() {
  let some_u8_value = Some(0u8);

  // using match
  match some_u8_value {
    Some(3) => println!("match: three"),
    _ => (),
  }

  // using if let
  // NOTE: you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
  if let Some(3) = some_u8_value {
    println!("if let: three")
  }
}
