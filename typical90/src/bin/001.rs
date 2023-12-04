use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,l:usize,k:usize,mut a:[usize;n],}

  a.push(l);
  for i in (0..n).rev() {
    a[i + 1] = a[i + 1] - a[i];
  }

  let is_ok = |s: usize| {
    let mut length = 0usize;
    let mut pieces = 0usize;
    for a in &a {
      length += a;
      if length >= s {
        pieces += 1;
        length = 0;
      }
    }
    pieces >= k + 1
  };
  let mut ok = 1usize;
  let mut ng = l;
  while ng - ok > 1 {
    let mid = (ok + ng) / 2;
    if is_ok(mid) {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  println!("{}", ok);
}
