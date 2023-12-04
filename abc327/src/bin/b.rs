use proconio::{fastout, input};

fn ipowi(i: u128) -> u128 {
  let mut ans = 1;
  for _ in 0..i {
    ans *= i;
  }
  ans
}

#[fastout]
fn main() {
  input! {b:u128}
  for i in 1..16 {
    if ipowi(i) == b {
      println!("{}", i);
      return;
    }
  }
  println!("-1");
}
