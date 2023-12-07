use ac_library::{Max, Segtree};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,q:usize,a:[i64;n]}

  let mut st = Segtree::<Max<i64>>::from(a);
  for _ in 0..q {
    input! {t:usize}
    match t {
      1 => {
        input! {x:Usize1,v:i64}
        st.set(x, v);
      }
      2 => {
        input! {l:Usize1,r:Usize1}
        println!("{}", st.prod(l..=r));
      }
      3 => {
        input! {x:Usize1,v:i64}
        println!("{}", st.max_right(x, |&f| f < v) + 1)
      }
      _ => (),
    }
  }
}
