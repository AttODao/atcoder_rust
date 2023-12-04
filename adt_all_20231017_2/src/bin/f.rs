use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {s:Chars,t:Chars}

  let mut i = 0;
  while i < s.len() && s[i] == t[i] {
    i += 1;
  }
  println!("{}", i + 1)
}
