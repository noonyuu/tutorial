fn main() {
  // println!("Hello, world!");

  // another_function();
  // another_function_with_parameters(5, 'x');


  // let x = {
  //   let y = 3;
  //   // ここに式を書くことで値を返すことができる
  //   // ;を書くと文になるからエラーになる
  //   y + 1
  // };

  // println!("x is {x}");

  // let x = five();
  let x = plus_one(5);
  println!("x is {x}");
}

// fn another_function() {
//   println!("Another function.");
// }

// fn another_function_with_parameters(x: i32, y: char) {
//   println!("Another function with parameters: {x}, {y}");
// }

// fn five() -> i32 {
//   5
// }

fn plus_one(x: i32) -> i32 {
    x + 1
}
