use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,m:usize,mut a:[usize;n],mut b:[usize;m]}

  a.push(1 << 30);
  b.push(1 << 30);
  let (mut ansa, mut ansb) = (vec![-1; n], vec![-1; m]);
  let (mut ia, mut ib) = (0, 0);
  let mut i = 1;
  while ia < n || ib < m {
    if a[ia] < b[ib] {
      ansa[ia] = i;
      ia += 1;
    } else {
      ansb[ib] = i;
      ib += 1;
    }
    i += 1;
  }
  for ansa in ansa {
    print!("{} ", ansa);
  }
  print!("\n");
  for ansb in ansb {
    print!("{} ", ansb);
  }
  print!("\n");
}
