// enum IpAddrKind {
//   V4,
//   V6,
// }

// enum values
enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String),
}

#[derive(Debug)]
enum Message {
  _Quit,
  _Move { x: i32, y: i32 },
  Write(String),
  _ChangeColor(i32, i32, i32),
}

// define Method
impl Message {
  fn call(&self) {
    println!("Message is called {:?}", &self);
  }
}

/*
 * Option enum
 *
 * use instead of null
 */
// enum Option<T> {
//   Some(T),
//   None,
// }

pub fn main() {
  let home = IpAddrKind::V4(127, 0, 0, 1);
  let loopback = IpAddrKind::V6(String::from("::1"));

  route(home);
  route(loopback);

  let m = Message::Write(String::from("hello"));
  m.call();

  let _some_number = Some(5);
  let _some_string = Some("a string");
  let _absent_number: Option<i32> = None;

  /*
   *  Error: no implementation for `i8 + std::option::Option<i8>`
   */
  // let x: i8 = 5;
  // let y: Option<i8> = Some(5);
  // let sum = x + y;
}

fn route(_ip_type: IpAddrKind) {}
