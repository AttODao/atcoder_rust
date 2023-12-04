use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}

  let mut zeros = 0;
  let mut ans = 0;
  for c in s {
    if c == '0' {
      zeros += 1;
    } else {
      ans += 1;
      if zeros > 0 {
        ans += (zeros + 1) / 2;
        zeros = 0;
      }
    }
  }
  ans += (zeros + 1) / 2;
  println!("{}", ans);
}
