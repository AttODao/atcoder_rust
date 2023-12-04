use proconio::{input, marker::Chars};

fn main() {
  input! {s:Chars}
  print!("{}", s.iter().map(|x| x.to_digit(10).unwrap()).sum::<u32>());
}
