use ac_library::MfGraph;
use itertools::{iproduct, Itertools};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,m:usize,mut s:[Chars;n]}

  let mut mf = MfGraph::<u32>::new(n * m + 2);
  for (i, j) in iproduct!(0..n, 0..m) {
    if s[i][j] == '.' {
      if (i ^ j) & 1 == 0 {
        if i < n - 1 && s[i + 1][j] == '.' {
          mf.add_edge(i + n * j, (i + 1) + n * j, 1);
        }
        if j < m - 1 && s[i][j + 1] == '.' {
          mf.add_edge(i + n * j, i + n * (j + 1), 1);
        }
      } else {
        if i < n - 1 && s[i + 1][j] == '.' {
          mf.add_edge((i + 1) + n * j, i + n * j, 1);
        }
        if j < m - 1 && s[i][j + 1] == '.' {
          mf.add_edge(i + n * (j + 1), i + n * j, 1);
        }
      }
    }
    if (i ^ j) & 1 == 0 {
      mf.add_edge(n * m, i + n * j, 1);
    } else {
      mf.add_edge(i + n * j, n * m + 1, 1);
    }
  }
  println!("{}", mf.flow(n * m, n * m + 1));
  for e in mf.edges() {
    if e.flow > 0 && e.from != n * m && e.to != n * m + 1 {
      let (p, q) = (e.from.min(e.to), e.from.max(e.to));
      if (q - p) % n == 0 {
        s[p % n][p / n] = '>';
        s[q % n][q / n] = '<';
      } else {
        s[p % n][p / n] = 'v';
        s[q % n][q / n] = '^';
      }
    }
  }
  println!(
    "{}",
    s.into_iter()
      .map(|s| s.iter().collect::<String>())
      .join("\n")
  );
}
