use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {mut n:usize}

  if n & 1 == 1 {
    println!("0");
  } else {
    n /= 10;
    let mut ans = n;
    while n > 0 {
      n /= 5;
      ans += n;
    }
    println!("{}", ans);
  }
}
