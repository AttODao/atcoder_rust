use ac_library::ModInt1000000007;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize}

  let mut fact = vec![ModInt1000000007::new(1)];
  (1..=n).for_each(|i| fact.push(*fact.last().unwrap() * i));
  for k in 1..=n {
    let mut ans = ModInt1000000007::new(0);
    for i in 1..=((n + k - 1) / k) {
      ans += fact[n - (k - 1) * (i - 1)] / fact[i] / fact[n - (k - 1) * (i - 1) - i];
    }
    println!("{}", ans);
  }
}
