use ac_library::Dsu;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {h:usize,w:usize,s:[Chars;h]}

  let mut uf = Dsu::new(h * w);
  let neighbors = |i: usize, j: usize| {
    let mut ret = vec![];
    if i < h - 1 {
      ret.push((i + 1, j));
      if j > 0 {
        ret.push((i + 1, j - 1));
      }
      if j < w - 1 {
        ret.push((i + 1, j + 1));
      }
    }
    if j < w - 1 {
      ret.push((i, j + 1));
    }
    ret
  };
  for i in 0..h {
    for j in 0..w {
      if s[i][j] == '#' {
        let pos = i * w + j;
        for (ni, nj) in neighbors(i, j) {
          if s[ni][nj] == '#' {
            let npos = ni * w + nj;
            uf.merge(pos, npos);
          }
        }
      }
    }
  }
  println!(
    "{}",
    uf.groups()
      .iter()
      .map(|p| if s[p[0] / w][p[0] % w] == '#' { 1 } else { 0 })
      .sum::<usize>()
  );
}
