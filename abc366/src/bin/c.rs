#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {q:usize}
  let mut ans = 0;
  let mut bag = vec![0; 1000010];
  for _ in 0..q {
    input! {t:u8}
    match t {
      1 => {
        input! {x:usize}
        if bag[x] == 0 {
          ans += 1;
        }
        bag[x] += 1;
      }
      2 => {
        input! {x:usize}
        bag[x] -= 1;
        if bag[x] == 0 {
          ans -= 1;
        }
      }
      3 => {
        println!("{}", ans);
      }
      _ => unreachable!(),
    };
  }
}
