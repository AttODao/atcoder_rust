use im_rc::HashSet;
use proconio::input;

fn main() {
  input! {n:u32,d:[u32;n]}
  print!("{}", d.into_iter().collect::<HashSet<u32>>().len());
}
