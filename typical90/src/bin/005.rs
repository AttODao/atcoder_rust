use ac_library::{pow_mod, ModInt1000000007};
use itertools::iproduct;
use proconio::{fastout, input};

fn merge(
  digits: usize,
  p: &Vec<ModInt1000000007>,
  q: &Vec<ModInt1000000007>,
) -> Vec<ModInt1000000007> {
  let b = p.len();
  let mut ret = vec![ModInt1000000007::new(0); b];
  let pow10 = pow_mod(10, digits as i64, b as u32);
  for (i, j) in iproduct!(0..b, 0..b) {
    ret[(i + j * pow10 as usize) % b] += p[i] * q[j];
  }
  ret
}

#[fastout]
fn main() {
  input! {n:usize,b:usize,k:usize,c:[usize;k]}

  let mut double = vec![ModInt1000000007::new(0); b];
  let mut dp = vec![ModInt1000000007::new(0); b];
  dp[0] = 1.into();
  for c in c {
    double[c % b] += 1;
  }
  let mut i = 1;
  while i <= n {
    if n & i > 0 {
      dp = merge(i, &double, &dp);
    }
    double = merge(i, &double, &double);
    i <<= 1;
  }
  println!("{}", dp[0]);
}
