use std::cmp::{max, min};

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:[Chars;10]}

  let (mut a, mut b, mut c, mut d) = (100, -100, 100, -100);
  for i in 0..10 {
    for j in 0..10 {
      if s[i][j] == '#' {
        a = min(a, i as i32);
        b = max(b, i as i32);
        c = min(c, j as i32);
        d = max(d, j as i32);
      }
    }
  }
  println!("{} {}\n{} {}", a + 1, b + 1, c + 1, d + 1);
}
