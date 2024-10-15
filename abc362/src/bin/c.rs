#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {lr:[(i64,i64)]}
  let (mut sl, sr) = lr.iter().fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
  if sl <= 0 && 0 <= sr {
    println!("Yes");
    for (l, r) in lr {
      let d = (r - l).min(-sl);
      print!("{} ", l + d);
      sl += d;
    }
  } else {
    println!("No");
  }
}
