use itertools::{izip, Itertools};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,s:[Chars;n]}

  println!(
    "{}",
    izip!(
      s.iter().map(|s| s.iter().filter(|&&c| c == 'x').count()),
      1..=n
    )
    .sorted()
    .unzip::<usize, usize, Vec<usize>, Vec<usize>>()
    .1
    .iter()
    .join(" ")
  );
}
