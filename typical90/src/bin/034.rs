use im_rc::HashMap;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,k:usize,a:[u32;n]}

  let mut s = HashMap::<u32, usize>::new();
  let mut l = 0;
  let mut cnt = 0;
  let mut ans = 0;
  for r in 0..n {
    *s.entry(a[r]).or_default() += 1;
    if s[&a[r]] == 1 {
      cnt += 1;
    }
    while cnt > k {
      s[&a[l]] -= 1;
      if s[&a[l]] == 0 {
        cnt -= 1;
      }
      l += 1;
    }
    ans = ans.max(r - l + 1);
  }
  println!("{}", ans);
}
