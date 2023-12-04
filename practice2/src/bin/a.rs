use ac_library::Dsu;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,q:usize,queries:[(usize,usize,usize);q]}

  let mut dsu = Dsu::new(n);
  for (t, u, v) in queries {
    if t == 0 {
      dsu.merge(u, v);
    } else {
      if dsu.same(u, v) {
        println!("1");
      } else {
        println!("0");
      }
    }
  }
}
