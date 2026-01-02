use std::io;

fn main() {
  // let guess: u32 = "42".parse().expect("Not a number");
  // println!("guess is {guess}");

  // let quotient = 56.7 / 32.2; // 56.7と32.2はf64型なので、商は1.7608695652173913になる
  // let floored = 2 / 3; // 2と3はu32型なので、商は0になる
  // println!("quotient is {quotient}");
  // println!("floored is {floored}");

  // ------------------------------- 文字 -------------------------------
  // let c = 'z';
  // let z = 'ℤ';
  // let heart_eyed_cat = '😻';
  // println!("c is {c}");
  // println!("z is {z}");
  // println!("heart_eyed_cat is {heart_eyed_cat}");

  // ------------------------------- タプル -------------------------------
  // tupに3つ組(タプル)としてはいる
  // let tup: (i32, f64, u8) = (500, 6.4, 1);
  // // タプルを分解してx, y, zの変数に代入
  // let (x, y, z) = (500, 6.4, 1);
  // println!("x is {x}");
  // println!("y is {y}");
  // println!("z is {z}");

  // let x: (i32, f64, u8) = (500, 6.4, 1);
  // let five_hundred = x.0;
  // let six_point_four = x.1;
  // let one = x.2;
  // println!("five_hundred is {five_hundred}");
  // println!("six_point_four is {six_point_four}");
  // println!("one is {one}");

  // ------------------------------- 配列 -------------------------------
  // 使用例: 1年の月の名前を扱うプログラムでは月を追加したり削除したりすることはまずないので配列を使用する
  // 柔軟に扱いたいケースではベクタ型を使用する
  // let a = [1, 2, 3, 4, 5];
  // let mouth = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
  let a = [1, 2, 3, 4, 5];
  // 角かっこの中に初期値とセミコロン、配列の長さを与えることで各要素に同じ値を持つように配列を初期化できる
  // ↓ let a = [3, 3, 3, 3, 3];と同じ
  // let a = [3; 5];

  println!("Please enter an array index.");

  let mut index = String::new();

  io::stdin()
      .read_line(&mut index)
      .expect("Failed to read line");

  let index: usize = index
      .trim()
      .parse()
      .expect("Index entered was not a number");

  let element = a[index];

  println!(
      "The value of the element at index {} is: {}",
      index, element
  );
}
