use itertools::{iproduct, Itertools};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,lr:[(usize,usize,usize,usize);n]}

  let mut grid = vec![vec![0i32; 1010]; 1010];
  for (lx, ly, rx, ry) in lr {
    grid[lx][ly] += 1;
    grid[lx][ry] -= 1;
    grid[rx][ly] -= 1;
    grid[rx][ry] += 1;
  }
  let mut ans = vec![0; n + 1];
  let mut sum = vec![vec![0; 1011]; 1011];
  for (i, j) in iproduct!(0..1010, 0..1010) {
    sum[i + 1][j + 1] = grid[i][j] + sum[i][j + 1] + sum[i + 1][j] - sum[i][j];
    ans[sum[i + 1][j + 1] as usize] += 1;
  }
  println!("{}", ans[1..].into_iter().join("\n"));
}
