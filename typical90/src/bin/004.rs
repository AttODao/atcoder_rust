use itertools::{iproduct, Itertools};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {h:usize,w:usize,a:[[usize;w];h]}

  let mut ls = vec![0; h];
  let mut cs = vec![0; w];
  for (i, j) in iproduct!(0..h, 0..w) {
    ls[i] += a[i][j];
    cs[j] += a[i][j];
  }
  println!(
    "{}",
    (0..h)
      .map(|i| (0..w).map(|j| ls[i] + cs[j] - a[i][j]).join(" "))
      .join("\n")
  )
}
