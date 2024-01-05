#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {_:usize,s:Chars}
  let mut nl = 0;
  println!(
    "{}",
    s.into_iter().fold(String::new(), |mut t, c| {
      if c == ')' && nl > 0 {
        while let Some(c) = t.pop() {
          if c == '(' {
            break;
          }
        }
        nl -= 1;
      } else {
        t.push(c);
        if c == '(' {
          nl += 1;
        }
      }
      t
    })
  )
}
