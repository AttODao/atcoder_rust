use std::collections::BTreeMap;

use ac_library::ModInt1000000007;
use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

type Mint = ModInt1000000007;

#[fastout]
fn main() {
  input! {mut h:usize,mut w:usize,mut c:[Chars;h]}
  let mut state_index = vec![BTreeMap::new(); w];
  let mut statenum = vec![0; w];
  for i in 0..w {
    let mut dfs = vec![(0u32, 0usize)];
    while let Some((state, depth)) = dfs.pop() {
      if depth == w + 1 {
        if state & (state >> w) == 0
          && (i == 0 || state & (state >> w - 1) & 1 == 0)
          && ((i + w - 1) % w == 0 || (state >> 1) & (state >> w) & 1 == 0)
        {
          state_index[i].insert(state, statenum[i]);
          statenum[i] += 1;
        }
      } else {
        if (depth + i + w - 1) % w == 0 || state & 1 == 0 {
          dfs.push((((state << 1) + 1), depth + 1));
        }
        dfs.push((state << 1, depth + 1));
      }
    }
  }
  let mut next0 = vec![];
  let mut next1 = vec![];
  for i in 0..w {
    let mut nex0 = vec![0; statenum[i]];
    let mut nex1 = vec![None; statenum[i]];
    for (state, index) in &state_index[i] {
      let state0 = (state << 1) & ((1 << w + 1) - 1);
      nex0[*index] = state_index[(i + 1) % w][&state0];
      let state1 = state0 + 1;
      if state_index[(i + 1) % w].contains_key(&state1) && (i == 0 || (state >> w) & 1 == 0) {
        nex1[*index] = Some(state_index[(i + 1) % w][&state1]);
      }
    }
    next0.push(nex0);
    next1.push(nex1);
  }

  let mut dp = vec![Mint::new(0); statenum[0]];
  dp[state_index[0][&0]] += 1;
  for (i, j) in iproduct!(0..h, 0..w) {
    let mut ndp = vec![Mint::new(0); statenum[(j + 1) % w]];
    (0..statenum[j]).for_each(|index| ndp[next0[j][index]] += dp[index]);
    if c[i][j] == '.' {
      (0..statenum[j]).for_each(|index| {
        if let Some(next1) = next1[j][index] {
          ndp[next1] += dp[index]
        }
      });
    }
    dp = ndp;
  }
  println!("{}", dp.into_iter().sum::<Mint>());
}
