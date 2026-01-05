fn main() {
  println!("Hello, world!");

  another_function();
  another_function_with_parameters(5, 'x');
}

fn another_function() {
  println!("Another function.");
}

fn another_function_with_parameters(x: i32, y: char) {
  println!("Another function with parameters: {x}, {y}");
}
