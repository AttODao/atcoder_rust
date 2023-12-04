use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,mut a:[usize;n]}

  a.sort_by(|a, b| b.cmp(a));
  let max = a[0];
  for a in a {
    if a != max {
      println!("{}", a);
      return;
    }
  }
}
