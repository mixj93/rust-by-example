/**
 * $ rustc hello.rs
 * $ ./hello
 */

fn main() {
  // println! 是一个宏 macros
  println!("Hello World");

  println!("{} days", 31); // 31默认是i32类型
  println!("{} days", 31i64);

  println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

  println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jump over");

  println!("{} of {:b} people know binary, the other half don't", 1, 2);
}
