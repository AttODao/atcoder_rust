use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,x:usize,p:[usize;n]}

  for k in 0..n {
    if x == p[k] {
      println!("{}", k + 1);
      return;
    }
  }
}
