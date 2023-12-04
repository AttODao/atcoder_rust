use ac_library::Dsu;
use proconio::{fastout, input, marker::Usize1};

fn get_neighbors(i: usize, j: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
  let mut ret = vec![];
  if i > 0 {
    ret.push((i - 1, j));
  }
  if i < h - 1 {
    ret.push((i + 1, j));
  }
  if j > 0 {
    ret.push((i, j - 1));
  }
  if j < w - 1 {
    ret.push((i, j + 1));
  }
  ret
}

fn get_node(i: usize, j: usize, h: usize) -> usize {
  i + j * h
}

#[fastout]
fn main() {
  input! {h:usize,w:usize,q:usize}

  let mut grid = vec![vec![false; w]; h];
  let mut uf = Dsu::new(h * w);
  for _ in 0..q {
    input! {t:usize}
    if t == 1 {
      input! {r:Usize1,c:Usize1}
      grid[r][c] = true;
      let node = get_node(r, c, h);
      for (i, j) in get_neighbors(r, c, h, w) {
        if grid[i][j] {
          uf.merge(node, get_node(i, j, h));
        }
      }
    } else {
      input! {ra:Usize1,ca:Usize1,rb:Usize1,cb:Usize1}
      println!(
        "{}",
        if grid[ra][ca] && grid[rb][cb] && uf.same(get_node(ra, ca, h), get_node(rb, cb, h)) {
          "Yes"
        } else {
          "No"
        }
      );
    }
  }
}
