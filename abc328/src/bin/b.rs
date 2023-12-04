use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize, d:[usize;n]}

  let mut ans = 0;
  for i in 1..10 {
    if i <= n && d[i - 1] >= i {
      ans += 1;
      if d[i - 1] >= 11 * i {
        ans += 1;
      }
    }
    if 11 * i <= n && d[11 * i - 1] >= i {
      ans += 1;
      if d[11 * i - 1] >= 11 * i {
        ans += 1;
      }
    }
  }
  println!("{}", ans);
}
