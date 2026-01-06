fn main() {
  // let number = 3;

  // if number < 5 {
  //   println!("condition was true");
  // } else {
  //   println!("condition was false");
  // }
  
  // let condition = true;
  // let number = if condition { 5 } else { 6 };
  // ng
  // この場合ifとelseの戻り値の型が一致しないためエラーになる
  // let number = if condition { 5 } else { "six" };
  // println!("number is {number}");

  // 無限ループ
  // loop {
  //   println!("again!");
  // }
  
  // let mut count = 0;
  // 'counting_up: loop {
  //   println!("count = {}", count);
  //   let mut remaining = 10;

  //   loop {
  //       println!("remaining = {}", remaining);
  //       if remaining == 9 {
  //           break;
  //       }
  //       if count == 2 {
  //           break 'counting_up;
  //       }
  //       remaining -= 1;
  //   }

  //   count += 1;
  // }
  // println!("End count = {}", count);

  // let mut number = 3;

  // while number != 0 {
  //     println!("{}!", number);

  //     number -= 1;
  // }

  // // 発射！
  // println!("LIFTOFF!!!");

  // let a = [10, 20, 30, 40, 50];
  // let mut index = 0;

  // while index < 5 {
  //   // 値は{}です
  //   println!("the value is: {}", a[index]);

  //   index += 1;
  // }

  // let a = [10, 20, 30, 40, 50];

  // for element in a {
  //     println!("the value is: {}", element);
  // }

  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}
