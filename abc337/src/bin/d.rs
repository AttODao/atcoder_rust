#![allow(non_snake_case)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {h:usize,w:usize,k:usize,s:[Chars;h]}
  let mut ans = u32::MAX;
  if w >= k {
    for i in 0..h {
      let mut cntx = vec![0; w + 1];
      let mut cntdot = vec![0; w + 1];
      for j in 0..w {
        cntx[j + 1] = cntx[j] + (s[i][j] == 'x') as u32;
        cntdot[j + 1] = cntdot[j] + (s[i][j] == '.') as u32;
      }
      for j in 0..=w - k {
        if cntx[j] == cntx[j + k] {
          ans = ans.min(cntdot[j + k] - cntdot[j]);
        }
      }
    }
  }
  if h >= k {
    for j in 0..w {
      let mut cntx = vec![0; h + 1];
      let mut cntdot = vec![0; h + 1];
      for i in 0..h {
        cntx[i + 1] = cntx[i] + (s[i][j] == 'x') as u32;
        cntdot[i + 1] = cntdot[i] + (s[i][j] == '.') as u32;
      }
      for i in 0..=h - k {
        if cntx[i] == cntx[i + k] {
          ans = ans.min(cntdot[i + k] - cntdot[i]);
        }
      }
    }
  }
  if ans == u32::MAX {
    println!("-1");
  } else {
    println!("{}", ans);
  }
}
