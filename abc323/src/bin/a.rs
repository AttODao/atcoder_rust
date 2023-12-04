use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}
  println!(
    "{}",
    if s.iter().skip(1).step_by(2).all(|&c| c == '0') {
      "Yes"
    } else {
      "No"
    }
  );
}
