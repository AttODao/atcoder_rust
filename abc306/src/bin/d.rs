#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {xy:[(u8,i64)]}
  let mut dp = (0, 0);
  for (x, y) in xy {
    if x == 0 {
      dp.0 = dp.0.max(dp.1.max(dp.0) + y);
    } else {
      dp.1 = dp.1.max(dp.0 + y);
    }
  }
  println!("{}", dp.0.max(dp.1));
}
