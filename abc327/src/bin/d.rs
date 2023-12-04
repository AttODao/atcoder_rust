use ac_library::TwoSat;
use itertools::izip;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize,a:[Usize1;m],b:[Usize1;m]}

  let mut ts = TwoSat::new(n);
  for (a, b) in izip!(a, b) {
    ts.add_clause(a, true, b, true);
    ts.add_clause(a, false, b, false);
  }
  if ts.satisfiable() {
    println!("Yes");
  } else {
    println!("No");
  }
}
