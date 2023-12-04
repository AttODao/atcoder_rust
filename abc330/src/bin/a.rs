use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,l:i32,a:[i32;n]}
  println!(
    "{}",
    a.iter().map(|&a| if a >= l { 1 } else { 0 }).sum::<i32>()
  );
}
