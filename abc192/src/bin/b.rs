use proconio::marker::Chars;
#[allow(non_camel_case_types, non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {s:Chars}

  if s
    .iter()
    .enumerate()
    .all(|(i, c)| (i % 2 == 0) == (c.is_lowercase()))
  {
    println!("Yes");
  } else {
    println!("No");
  }
}
