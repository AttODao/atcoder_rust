use proconio::input;

fn main() {
  input! {n:i32,a:[i32;n],}
  print!(
    "{}",
    a.iter()
      .map(|a| {
        let mut ret = 0;
        while (a >> ret) & 1 == 0 {
          ret += 1;
        }
        ret
      })
      .min()
      .unwrap()
  );
}
