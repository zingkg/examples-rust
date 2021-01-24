mod bit_vector;
mod hamming_dist;
mod sieve_of_eratosthenes;

fn main() {
  let result = sieve_of_eratosthenes::calculate_primes(500);
  for i in result {
    print!("{} ", i);
  }
  println!("\nCompleted");

  let result2 = hamming_dist::hamming_dist("abc".to_string(), "abc".to_string());
  println!("result2 {}", result2);

  println!("result3 {}", vec![1; 4].len());
}
