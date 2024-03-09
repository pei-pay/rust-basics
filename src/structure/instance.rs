struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

pub fn main() {
  let mut user1 = User {
    email: String::from("test@example.com"),
    username: String::from("test01"),
    active: true,
    sign_in_count: 1,
  };

  user1.email = String::from("test1@example.com");

  let user2 = build_user(String::from("test2@example.com"), String::from("test02"));
  println!(
    "user2: username is {}, sign_in_count is {}, active is {}",
    user2.username, user2.sign_in_count, user2.active
  );

  // Creating Instances from Other Instances with Struct Update Syntax
  let user3 = User {
    email: String::from("test3@example.com"),
    username: String::from("test03"),
    ..user2
  };

  println!("user3: username is {}", user3.username)
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
