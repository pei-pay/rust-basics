pub fn main() {
  let v: Vec<i32> = Vec::new();

  let v2 = vec![1, 2, 3];

  /*
   * accessing a value
   */
  //  with indexing syntax
  let third: &i32 = &v2[2];
  println!("The third element is {}", third);
  // with get method
  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
  }

  let mut v3 = Vec::new();
  v3.push(5);
  v3.push(6);
  v3.push(7);
  v3.push(8);
}
