use ac_library::TwoSat;
use itertools::{iproduct, Itertools};
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,d:u64,xy:[(i64,i64);n]}

  let mut ts = TwoSat::new(n);
  for (i, j) in (0..n).tuple_combinations() {
    let ((xi, yi), (xj, yj)) = (xy[i], xy[j]);
    if xi.abs_diff(xj) < d {
      ts.add_clause(i, false, j, false);
    }
    if xi.abs_diff(yj) < d {
      ts.add_clause(i, false, j, true);
    }
    if yi.abs_diff(xj) < d {
      ts.add_clause(i, true, j, false);
    }
    if yi.abs_diff(yj) < d {
      ts.add_clause(i, true, j, true);
    }
  }
  if ts.satisfiable() {
    println!(
      "Yes\n{}",
      ts.answer()
        .into_iter()
        .zip(xy)
        .map(|(&flag, (x, y))| if flag { x } else { y })
        .join("\n")
    );
  } else {
    println!("No");
  }
}
