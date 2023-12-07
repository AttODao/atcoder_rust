use ac_library::{convolution, ModInt998244353};
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[u64;n],b:[u64;m]}

  let a = a.into_iter().map(ModInt998244353::new).collect_vec();
  let b = b.into_iter().map(ModInt998244353::new).collect_vec();
  println!(
    "{}",
    convolution(&a, &b)
      .iter()
      .map(ModInt998244353::to_string)
      .join(" ")
  );
}
