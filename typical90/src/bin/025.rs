use im_rc::HashSet;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:u64,b:u64}

  let mut fm = HashSet::new();
  (0..10).combinations_with_replacement(10).for_each(|s| {
    fm.insert(s.into_iter().product::<u64>());
  });
  let mut ans = 0u64;
  for fm in fm {
    let mut m = fm + b;
    if m <= n {
      let mut nfm = 1;
      while m > 0 {
        nfm *= m % 10;
        m /= 10;
      }
      if nfm == fm {
        ans += 1;
      }
    }
  }
  println!("{}", ans);
}
