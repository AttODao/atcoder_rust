use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars}

  let s = s.iter().map(|c| c.to_digit(10).unwrap() as usize);
  let mut table = [true; 10];
  for n in s {
    table[n] = false;
  }
  for n in 0..10 {
    if table[n] {
      println!("{}", n);
      return;
    }
  }
}
