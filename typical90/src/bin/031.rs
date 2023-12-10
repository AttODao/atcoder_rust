use itertools::{iproduct, izip};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,w:[usize;n],b:[usize;n]}

  let mut gr = [[0; 1376]; 51];
  for (w, b) in iproduct!(0..51, 0..1326) {
    let mut s = [true; 1501];
    if w >= 1 {
      s[gr[w - 1][b + w]] = false;
    }
    if b >= 2 {
      for k in 1..=b / 2 {
        s[gr[w][b - k]] = false;
      }
    }
    gr[w][b] = s.iter().position(|&s| s).unwrap();
  }

  let mut xor = 0;
  for (w, b) in izip!(w, b) {
    xor ^= gr[w][b];
  }
  println!("{}", if xor > 0 { "First" } else { "Second" });
}
