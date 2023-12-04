use std::cmp::max;

use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,wx:[(usize,usize);n]}

  let mut ans = 0;
  for i in 0..24 {
    let mut cnt = 0;
    for &(w, x) in wx.iter() {
      let time = (i + x) % 24;
      if 9 <= time && time <= 17 {
        cnt += w;
      }
    }
    ans = max(ans, cnt);
  }
  println!("{}", ans);
}
