use itertools::{iproduct, izip, Itertools};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,row:Chars,col:Chars}

  for (a, b, c) in iproduct!(
    (0..n).permutations(n),
    (0..n).permutations(n),
    (0..n).permutations(n)
  ) {
    let mut grid = vec![vec!['.'; n]; n];
    let mut is_valid = true;
    for (g, aj, bj, cj) in izip!(&mut grid, a, b, c) {
      g[aj] = 'A';
      g[bj] = 'B';
      g[cj] = 'C';
      if bj == cj || cj == aj || aj == bj {
        is_valid = false;
        break;
      }
    }
    for (i, &c) in row.iter().enumerate() {
      for j in 0..n {
        if grid[i][j] != '.' {
          if grid[i][j] != c {
            is_valid = false;
          }
          break;
        }
      }
    }
    for (j, &c) in col.iter().enumerate() {
      for i in 0..n {
        if grid[i][j] != '.' {
          if grid[i][j] != c {
            is_valid = false;
          }
          break;
        }
      }
    }
    if is_valid {
      println!("Yes");
      for g in grid {
        for c in g {
          print!("{}", c);
        }
        print!("\n");
      }
      return;
    }
  }
  println!("No");
}
