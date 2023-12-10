use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,k:u64}

  let mut pf = vec![0u64; n + 1];
  for i in 2..=n {
    if pf[i] == 0 {
      (i..=n).step_by(i).for_each(|j| {
        pf[j] += 1;
      });
    }
  }
  println!("{}", pf.into_iter().filter(|&pf| pf >= k).count());
}
