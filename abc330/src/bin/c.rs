use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {d:i64}

  let mut x = 0;
  let mut ans = i64::MAX;
  while x * x <= d / 2 {
    let (mut l, mut r) = (0, 1420000);
    while r - l > 1 {
      let mid = (l + r) / 2;
      if x * x + mid * mid <= d {
        l = mid;
      } else {
        r = mid;
      }
    }
    ans = ans.min((d - x * x - l * l).min(x * x + r * r - d));
    x += 1;
  }
  println!("{}", ans);
}
