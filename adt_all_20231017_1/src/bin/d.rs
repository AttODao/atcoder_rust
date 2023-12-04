use ndarray::Array;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {n:usize,m:usize}

  let mut table = vec![0usize; n * m];
  for i in 0..m {
    input! {k:usize,x:[Usize1;k]}
    for x in x {
      table[i + x * m] = 1;
    }
  }

  let matrix = Array::from_shape_vec((n, m), table).unwrap();
  let prod = matrix.dot(&matrix.t());
  if prod.iter().all(|x| *x > 0) {
    println!("Yes");
  } else {
    println!("No");
  }
}
