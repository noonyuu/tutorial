fn main() {
  let s1 = String::from("hello");
  let s2 = s1;
  // println!("s1 is {}", s1); // これはすでにs1はmoveされているのでエラーになる
  println!("s2 is {}", s2); // これは正常に動作する

  // cloneを使うと所有権をコピーする
  let s3 = s2.clone();
  println!("s3 is {}", s3);
}
