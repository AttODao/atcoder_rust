#[allow(non_camel_case_types, non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {x:isize}
  println!("{}", x / 100 * 100 - x + 100);
}
