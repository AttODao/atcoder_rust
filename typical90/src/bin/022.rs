use num_integer::Integer;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:usize,b:usize,c:usize}

  let gcd = a.gcd(&b.gcd(&c));
  println!("{}", (a + b + c) / gcd - 3);
}
