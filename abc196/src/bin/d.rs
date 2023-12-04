use proconio::input;

fn dfs(x: usize, y: usize, a: usize, (h, w): (&usize, &usize), grid: &mut Vec<Vec<bool>>) -> usize {
  if a == 0 {
    return 1;
  }
  if y >= *h {
    return 0;
  }
  if x >= *w {
    return dfs(0, y + 1, a, (h, w), grid);
  }
  if grid[x][y] {
    return dfs(x + 1, y, a, (h, w), grid);
  }

  let mut ret = dfs(x + 1, y, a, (h, w), grid);
  if x < w - 1 && !grid[x + 1][y] {
    grid[x + 1][y] = true;
    ret += dfs(x + 2, y, a - 1, (h, w), grid);
    grid[x + 1][y] = false;
  }
  if y < h - 1 && !grid[x][y + 1] {
    grid[x][y + 1] = true;
    ret += dfs(x + 1, y, a - 1, (h, w), grid);
    grid[x][y + 1] = false;
  }
  ret
}

fn main() {
  input! {h:usize,w:usize,a:usize,b:usize}

  print!("{}", dfs(0, 0, a, (&h, &w), &mut vec![vec![false; h]; w]));
}
