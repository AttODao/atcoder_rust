use num_integer::lcm;
use proconio::{input, fastout};

#[fastout]
fn main() {
  input! {a:usize,b:usize}
  println!("{}",lcm(a, b));
}
