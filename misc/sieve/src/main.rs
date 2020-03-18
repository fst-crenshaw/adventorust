extern crate num;

use num::{one, range_inclusive, zero, Num};
use std::collections::HashSet;

fn is_prime<T>(n: T) -> bool
where
    T: Copy + num::ToPrimitive + num::CheckedAdd + std::cmp::PartialOrd + std::clone::Clone + Num,
{
    // Create a 0, 1, and 2 that are compatible with any numeric type.
    let any_zero: T = zero();
    let any_one: T = one();
    let any_two: T = any_one.checked_add(&any_one).unwrap();

    // Handle the base case.
    if n == any_zero || n == any_one {
        return false;
    }

    let range = range_inclusive(any_two, n);
    let mut primes = Vec::new();
    primes.push(false); // 0 is not prime.
    primes.push(false); // 1 is not prime.

    // At the start of time, presume that all numbers
    // starting from 2 are prime numbers.
    for _ in range {
        primes.push(true);
    }

    let range_again = range_inclusive(any_two, n);
    for i in range_again {
        let index: usize = i.to_usize().unwrap();

        if (primes[index] == true) && (i * i <= n) {
            let mut j = i * i;
            while j <= n {
                primes[j.to_usize().unwrap()] = false;
                j = j + i;
            }
        }
    }

    println!("{:?}", primes);
    return primes[n.to_usize().unwrap()];
}

/// A Rust implementation of the sieve of eratosthenes. Given a value,
/// n, return a HashSet indicating all of the prime numbers from 2 up
/// to n.  Works on values of type usize.
fn sieve(n: usize) -> HashSet<usize> {
    if matches!(n, 0..=1) {
        return HashSet::new();
    }

    // At the start of time, presume that all numbers
    // starting from 2 are prime numbers.
    let mut is_prime = (2..=n).collect::<HashSet<usize>>();

    for i in 2..=n {
        if is_prime.contains(&i) && i * i <= n {
            let mut j = i * i;
            while j <= n {
                is_prime.remove(&j);
                j = j + i;
            }
        }
    }
    is_prime
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{is_prime, sieve};
    use std::collections::HashSet;

    #[test]
    fn test_is_prime_base_case() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
    }

    #[test]
    fn test_is_prime_interesting_cases() {
        assert_eq!(is_prime(2 as usize), true);
        assert_eq!(is_prime(2 as u8), true);
        assert_eq!(is_prime(2 as u16), true);
        assert_eq!(is_prime(2 as u32), true);
        assert_eq!(is_prime(2 as u64), true);

        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(89), true);
        assert_eq!(is_prime(97), true);

        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(16), false);
        assert_eq!(is_prime(32), false);
    }

    #[test]
    fn test_sieve_base_case() {
        assert!(sieve(0).is_empty());
        assert!(sieve(1).is_empty());
    }

    #[test]
    fn test_sieve_interesting_cases() {
        let mut expected: HashSet<usize>;

        expected = [2].iter().cloned().collect();
        assert_eq!(sieve(2), expected);

        expected = [2, 3, 5, 7].iter().cloned().collect();
        assert_eq!(sieve(7), expected);

        expected = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(sieve(97), expected);
    }
}
