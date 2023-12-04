use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,cp:[(Usize1,i32);n],q:usize,lr:[(Usize1,Usize1);q]}

  let mut sum = vec![vec![0; 2]; n + 1];
  for (i, (c, p)) in cp.into_iter().enumerate() {
    sum[i + 1] = sum[i].clone();
    sum[i + 1][c] += p;
  }
  for (l, r) in lr {
    println!(
      "{} {}",
      sum[r + 1][0] - sum[l][0],
      sum[r + 1][1] - sum[l][1]
    );
  }
}
