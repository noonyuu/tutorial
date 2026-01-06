fn main() {
  // let number = 3;

  // if number < 5 {
  //   println!("condition was true");
  // } else {
  //   println!("condition was false");
  // }
  
  let condition = true;
  let number = if condition { 5 } else { 6 };
  // ng
  // この場合ifとelseの戻り値の型が一致しないためエラーになる
  // let number = if condition { 5 } else { "six" };
  println!("number is {number}");
}
