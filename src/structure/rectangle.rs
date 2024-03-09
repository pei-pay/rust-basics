/*
 * Let’s write a program that calculates the area of a rectangle.
 */

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

pub fn main() {
  //  basic
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle1 is {} square pixels.",
    area1(width1, height1)
  );

  // using tuple
  let rect1 = (30, 50);
  println!(
    "The area of the rectangle2 is {} square pixels.",
    area2(rect1)
  );

  // using struct
  let rect2 = Rectangle {
    width: 30,
    height: 50,
  };
  println!("rect2 is {:?}", rect2);
  println!(
    "The area of the rectangle2 is {} square pixels.",
    area3(&rect2)
  );
}

fn area1(width: u32, height: u32) -> u32 {
  width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area3(dimensions: &Rectangle) -> u32 {
  dimensions.width * dimensions.height
}
