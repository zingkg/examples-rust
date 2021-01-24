pub fn hamming_dist(a: std::string::String, b: std::string::String) -> usize {
  let max_length = std::cmp::max(a.len(), b.len());
  let min_length = std::cmp::min(a.len(), b.len());

  let padding = max_length - min_length;
  let a_vec: Vec<char> = a.chars().collect();
  let b_vec: Vec<char> = b.chars().collect();

  let mut same_char = 0;
  for i in 0..min_length {
    if a_vec[i] == b_vec[i] {
      same_char += 1;
    }
  }
  return min_length - same_char + padding;
}

#[cfg(test)]
mod hamming_dist {
  use super::*;

  #[test]
  fn hamming_dist_successfully() {
    let zero = hamming_dist("abc".to_string(), "abc".to_string());
    assert_eq!(zero, 0);

    let pad_a = hamming_dist("abc".to_string(), "abcde".to_string());
    assert_eq!(pad_a, 2);

    let pad_b = hamming_dist("abcde".to_string(), "abc".to_string());
    assert_eq!(pad_b, 2);

    let pad_same = hamming_dist("abc".to_string(), "xyz".to_string());
    assert_eq!(pad_same, 3);
  }
}
