pub fn check_hash(mut complexity: u32, hash: String) -> bool {
  let bit_compare = 1 << 127;
  let mut sum = u128::from_str_radix(&*hash, 16).unwrap();
  while complexity > 0 {
      if (sum & bit_compare) > 0 {
          break;
      }
      sum = sum << 1;
      complexity -= 1;
  }
  complexity == 0
}
