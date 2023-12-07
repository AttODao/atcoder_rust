#![allow(non_snake_case)]

use proconio::{fastout, input};

fn f(n: u128, x: u128, k: i128) -> u128 {
  if k > 70 {
    return 0;
  }
  match k.cmp(&0) {
    std::cmp::Ordering::Less => 0,
    std::cmp::Ordering::Equal => 1,
    std::cmp::Ordering::Greater => {
      let l = x << k;
      let r = (n + 1).min((x + 1) << k);
      if r > l {
        r - l
      } else {
        0
      }
    }
  }
}

#[fastout]
fn main() {
  input! {t:usize,nxk:[(u128,u128,i128);t]}

  for (n, x, k) in nxk {
    let mut ans = f(n, x, k);
    let (mut x, mut k) = (x, k);
    while x > 1 && k > 0 {
      ans += f(n, x / 2, k - 1) - f(n, x, k - 2);
      x /= 2;
      k -= 1;
    }

    println!("{}", ans);
  }
}
