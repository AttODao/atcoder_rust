use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:i64,a:i64,b:i64,c:i64}

  let mut ans = i64::MAX;
  for i in 0..10000 {
    for j in 0..(10000 - i) {
      if n - a * i - b * j >= 0 && (n - a * i - b * j) % c == 0 {
        let k = (n - a * i - b * j) / c;
        ans = ans.min(i + j + k);
      }
    }
  }
  println!("{}", ans);
}
