#[derive(Copy, Clone)]
enum PrimeStatus {
  Prime,
  NonPrime,
  Uncalculated
}

pub fn calculate_primes(through: usize) -> std::vec::Vec<usize> {
  let size = through + 1;
  let mut prime_status = vec![PrimeStatus::Uncalculated; size];
  for i in 2..size {
    match prime_status[i] {
      PrimeStatus::Uncalculated => {
        if test_prime(i) {
          prime_status[i] = PrimeStatus::Prime;
          let mut j = 2;
          while i * j < size {
            prime_status[i * j] = PrimeStatus::NonPrime;
            j += 1;
          }
        }
      }
      PrimeStatus::Prime | PrimeStatus::NonPrime => ()
    }
  }
  let mut result: std::vec::Vec<usize> = vec![];
  for i in 2..prime_status.len() {
    match prime_status[i] {
      PrimeStatus::Prime =>
        result.push(i),
      PrimeStatus::NonPrime => (),
      PrimeStatus::Uncalculated =>
        panic!("All values should have been calculated"),
    }
  }
  return result;
}

fn test_prime(x: usize) -> bool {
  for i in 2..x {
    if x % i == 0 {
      return false;
    }
  }

  return true;
}

#[cfg(test)]
mod calculate_primes {
  use super::*;

  #[test]
  fn calculate_primes_successfully() {
    let one = calculate_primes(1);
    assert_eq!(one, vec![]);

    let two = calculate_primes(2);
    assert_eq!(two, vec![2]);

    let eleven = calculate_primes(11);
    assert_eq!(eleven, vec![2, 3, 5, 7, 11]);

    let twenty = calculate_primes(20);
    assert_eq!(twenty, vec![2, 3, 5, 7, 11, 13, 17, 19]);
  }

  #[test]
  fn test_prime_expectations() {
    assert_eq!(test_prime(2), true);
    assert_eq!(test_prime(3), true);
    assert_eq!(test_prime(4), false);
    assert_eq!(test_prime(100), false);
  }
}
