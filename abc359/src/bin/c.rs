#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {sx:usize,sy:usize,tx:usize,ty:usize}
  let (x, y) = (
    (sx - ((sx + sy) & 1)).abs_diff(tx - ((tx + ty) & 1)),
    (ty.abs_diff(sy)),
  );
  println!("{}", if x < y { y } else { (x + y) / 2 });
}
