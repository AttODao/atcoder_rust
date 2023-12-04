use ac_library::Dsu;
use proconio::{fastout, input};

const H: usize = 2010;
const W: usize = 2010;

fn neighbors(pos: usize) -> Vec<usize> {
  return vec![pos - 1 - W, pos - W, pos - 1, pos + 1, pos + W, pos + 1 + W];
}

#[fastout]
fn main() {
  input! {n:usize,xy:[(i32,i32);n]}

  let mut uf = Dsu::new(H * W);
  let mut black = vec![false; H * W];
  for (x, y) in xy {
    let pos = (x + 1001 + (y + 1001) * W as i32) as usize;
    black[pos] = true;
    for nei in neighbors(pos) {
      if black[nei] {
        uf.merge(pos, nei);
      }
    }
  }
  println!(
    "{}",
    uf.groups()
      .iter()
      .map(|g| if black[g[0]] { 1 } else { 0 })
      .sum::<usize>()
  )
}
