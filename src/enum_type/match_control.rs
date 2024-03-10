enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

pub fn main() {
  let penny = Coin::Penny;
  let _nickel = Coin::Nickel;
  let dime = Coin::Dime;
  let _quarter = Coin::Quarter;

  value_in_cents(penny);
  value_in_cents(dime);

  let five = Some(5);
  let _six = plus_one(five);
  let _none = plus_one(None);

  let some_u8_value = 0u8;
  match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
  }
}

fn value_in_cents(coin: Coin) -> i32 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}
