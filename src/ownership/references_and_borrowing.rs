/*
 * ===============================
 * The Rules of References
 * - At any given time, you can have either one mutable reference or any number of immutable references.
 * - References must always be valid.
 * ===============================
 */

pub fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of '{}' is {}.", s1, len);

  /*
   * Mutable reference
   */
  // Not allowed to modify something we have a reference to.
  // change1(&s1);

  let mut s2: String = String::from("mutable");
  change2(&mut s2);
  println!("The value of 's2' is {}.", s2);

  // Error: cannot borrow `s2` as mutable more than once at a time
  // let r1 = &mut s2;
  // let r2 = &mut s2;

  // Error: cannot borrow `s2` as mutable because it is also borrowed as immutable
  // let r1 = &s2; // no problem
  // let r2 = &s2; // no problem
  // let r3 = &mut s2; // BIG PROBLEM

  /*
   * Dangling reference
   */
  // Error: missing lifetime specifier
  // let reference_to_nothing = dangle();
  let s3 = no_dangle();
  println!("s3 is not dangled s3: {}", s3)
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

// fn change1(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change2(some_string: &mut String) {
  some_string.push_str("world");
}

// fn dangle() -> &String { // dangle returns a reference to a String
//   let s = String::from("hello");
//   &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.

// -> The solution here is to return the String directly:
fn no_dangle() -> String {
  let s = String::from("hello");
  s
}
