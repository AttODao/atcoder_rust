use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
  input! {a:[[Usize1;9];9]}

  let mut used = [[[false; 9]; 3]; 3];
  for i in 0..9 {
    let mut rused = [false; 9];
    let mut cused = [false; 9];
    for j in 0..9 {
      if used[i / 3][j / 3][a[i][j]] || rused[a[i][j]] || cused[a[j][i]] {
        println!("No");
        return;
      }
      used[i / 3][j / 3][a[i][j]] = true;
      rused[a[i][j]] = true;
      cused[a[j][i]] = true;
    }
  }
  println!("Yes");
}
