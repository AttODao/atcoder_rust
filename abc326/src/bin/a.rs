use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {x:isize,y:isize}
  if x - 3 <= y && y <= x + 2 {
    println!("Yes");
  } else {
    println!("No");
  }
}
