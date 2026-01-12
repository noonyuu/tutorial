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

  // 戻り値とスコープ
  let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1にムーブする
  let s2 = String::from("hello");     // s2がスコープに入る
  let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ戻り値もs3にムーブされる
} 

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                             // 呼び出した関数にムーブする
    let some_string = String::from("hello"); // some_stringがスコープに入る
    some_string                              // some_stringが返され、呼び出し元関数に
                                             // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。
    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}