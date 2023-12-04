use proconio::input;

fn main() {
  input! {a:u32,b:u32,c:u32,x:u32}

  let mut ans = 0;
  for i in (0..=a).map(|i| 500 * i) {
    if i <= x {
      for j in (0..=b).map(|j| 100 * j) {
        if i + j <= x {
          for k in (0..=c).map(|k| 50 * k) {
            if i + j + k == x {
              ans += 1;
            }
          }
        }
      }
    }
  }

  print!("{}", ans);
}
