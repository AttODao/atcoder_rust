use ::ac_library::FenwickTree;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,q:usize,a:[usize;n],queries:[(usize,usize,usize);q]}

  let mut ft = FenwickTree::new(n, 0);
  for i in 0..n {
    ft.add(i, a[i]);
  }
  for (t, x, y) in queries {
    if t == 0 {
      ft.add(x, y);
    } else {
      println!("{}", ft.sum(x..y));
    }
  }
}
