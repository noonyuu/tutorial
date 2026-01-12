fn main() {
  let s1 = String::from("hello");
  let s2 = s1;
  // println!("s1 is {}", s1); // これはすでにs1はmoveされているのでエラーになる
  println!("s2 is {}", s2); // これは正常に動作する

  // cloneを使うと所有権をコピーする
  let s3 = s2.clone();
  println!("s3 is {}", s3);

  // ------------------------------- 所有権の移動 -------------------------------
  // 所有権と関数
  let s = String::from("hello");  // sがスコープに入る

  takes_ownership(s);   // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

  let x = 5;                   // xがスコープに入る

  makes_copy(x);      // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても大丈夫

  // Stringは所有権を持つので、関数に渡すと所有権が移動する
  // i32はCopy トレイトを持つので、関数に渡すと値がコピーされる

} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。