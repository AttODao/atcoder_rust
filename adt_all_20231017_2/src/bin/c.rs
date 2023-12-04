use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,p:usize,a:[usize;n]}
  println!(
    "{}",
    a.iter().map(|a| if a < &p { 1 } else { 0 }).sum::<usize>()
  );
}
