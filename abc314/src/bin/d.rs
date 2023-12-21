#![allow(non_snake_case)]

use proconio::{
  fastout, input,
  marker::{Chars, Usize1},
};

#[fastout]
fn main() {
  input! {n:usize,mut s:Chars,q:usize,txc:[(u8,usize,char);q]}

  if let Some(last) = (0..q).rev().skip_while(|&i| txc[i].0 == 1).next() {
    for &(t, x, c) in &txc[..last] {
      if t == 1 {
        s[x - 1] = c;
      }
    }
    s.iter_mut().for_each(if txc[last].0 == 2 {
      |s: &mut char| *s = s.to_lowercase().next().unwrap()
    } else {
      |s: &mut char| *s = s.to_uppercase().next().unwrap()
    });
    for &(_, x, c) in &txc[last + 1..] {
      s[x - 1] = c;
    }
  } else {
    for &(_, x, c) in &txc {
      s[x - 1] = c;
    }
  }
  println!("{}", s.iter().collect::<String>())
}
