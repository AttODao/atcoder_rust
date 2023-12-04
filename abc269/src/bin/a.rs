use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {a:isize,b:isize,c:isize,d:isize}

  println!("{}\nTakahashi", (a + b) * (c - d));
}
