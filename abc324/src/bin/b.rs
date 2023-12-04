use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {mut n:u64}
  while n % 2 == 0 {
    n /= 2;
  }
  while n % 3 == 0 {
    n /= 3;
  }
  println!("{}", if n == 1 { "Yes" } else { "No" });
}
