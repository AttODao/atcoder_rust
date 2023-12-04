use im_rc::HashMap;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {n:usize,sc:[(usize,usize);n]}

  let mut slimes: HashMap<usize, usize> = HashMap::new();
  for (s, c) in sc {
    let mut i = 1;
    while s & i == 0 {
      i <<= 1;
    }
    *slimes.entry(s / i).or_insert(0) += c * i;
  }
  println!("{}", slimes.values().map(|x| x.count_ones()).sum::<u32>());
}
