#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {h:[i64]}
  let mut t = 0;
  for mut h in h {
    t += h / 5 * 3;
    h %= 5;
    while h > 0 {
      t += 1;
      h -= if t % 3 == 0 { 3 } else { 1 };
    }
  }
  println!("{t}");
}
