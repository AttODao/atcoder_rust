use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,s:[Chars;n]}

  let mut cntr = vec![0; n];
  let mut cntc = vec![0; n];
  for (i, j) in iproduct!(0..n, 0..n) {
    if s[i][j] == 'o' {
      cntr[i] += 1;
      cntc[j] += 1;
    }
  }
  let mut ans: i64 = 0;
  for (i, j) in iproduct!(0..n, 0..n) {
    if s[i][j] == 'o' {
      ans += (cntr[i] - 1) * (cntc[j] - 1);
    }
  }
  println!("{}", ans);
}
