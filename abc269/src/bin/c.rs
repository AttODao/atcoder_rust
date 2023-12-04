use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:u64}

  let mut ans: Vec<u64> = vec![0];
  for i in 0..60 {
    if n >> i & 1 == 1 {
      let len = ans.len();
      for j in 0..len {
        ans.push(ans[j] + (1 << i));
      }
    }
  }
  for a in ans {
    println!("{}", a);
  }
}
