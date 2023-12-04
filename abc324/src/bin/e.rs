use itertools::{izip, Itertools};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
  input! {n:usize,mut t:Chars,s:[Chars;n]}

  let sl1 = s
    .iter()
    .map(|s| {
      let mut ret = 0;
      for &s in s {
        if t[ret] == s {
          ret += 1;
          if ret >= t.len() {
            break;
          }
        }
      }
      ret
    })
    .collect_vec();
  let sl2 = s
    .iter()
    .map(|s| {
      let mut ret = t.len();
      for &s in s.iter().rev() {
        if t[ret - 1] == s {
          ret -= 1;
          if ret == 0 {
            break;
          }
        }
      }
      ret
    })
    .collect_vec();
  let mut sum2 = vec![0; t.len() + 1];
  for sl2 in sl2 {
    sum2[sl2] += 1;
  }
  for i in 0..t.len() {
    sum2[i + 1] += sum2[i];
  }
  println!("{}", sl1.iter().map(|&i| sum2[i]).sum::<usize>());
}
