use std::collections::HashSet;

fn sieve<T>(segment_end: T) -> HashSet<T> {
    if matches!(segment_end, 0 as T..=1 as T) {
        return HashSet::new();
    }

    // At the start of time, presume that all numbers
    // starting from 2 are prime numbers.
    let mut is_prime = (2..=segment_end).collect::<HashSet<T>>();

    /*
    int n;
    vector<char> is_prime(n+1, true);
    is_prime[0] = is_prime[1] = false;
    for (int i = 2; i <= n; i++) {
    if (is_prime[i] && (long long)i * i <= n) {
        for (int j = i * i; j <= n; j += i)
            is_prime[j] = false;
        }
     }
     */
    for i in 2..=segment_end {
        if is_prime.contains(&i) && i * i <= segment_end {
            let mut j = i * i;
            while j < segment_end {
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
    use crate::sieve;
    use std::collections::HashSet;

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
