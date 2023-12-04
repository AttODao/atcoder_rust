use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,a:[usize;n]}

  let mut ans = 0;
  for a in a {
    if a == ans + 1 {
      ans = a;
    }
  }

  if ans == 0 {
    println!("-1");
  } else {
    println!("{}", n - ans);
  }
}
