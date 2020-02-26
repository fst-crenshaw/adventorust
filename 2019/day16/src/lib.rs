#[allow(unused_imports)]
use std::fs;
use std::thread;

/// Perform a phase of our FFT process.
pub fn fft_phase(signal: &Vec<i32>) -> Vec<i32> {
    signal
        .iter()
        .enumerate()
        .map(|(i, _)| RepeatingPatternIterator::new(i, 0))
        .map(|repeating_pattern| {
            let mut output_digit = 0;
            for (digit, pattern_element) in (*signal).iter().zip(repeating_pattern) {
                output_digit += digit * pattern_element;
            }
            output_digit
        })
        .map(|digit| digit.abs() % 10)
        .collect::<Vec<_>>()
}

pub fn fft_phase_in_parts(signal: &Vec<i32>, start: usize, len: usize) -> Vec<i32> {
    println!("start: {}", start);
    signal
        .iter()
        .skip(start)
        .take(len)
        .enumerate()
        .map(|(i, v)| {
            println!("i = {}, v= {}", i, v);
            let repeating_pattern = RepeatingPatternIterator::new(i + start, start);
            let mut output_digit = 0;
            for (digit, pattern_element) in (*signal).iter().zip(repeating_pattern) {
                println!("output_digit ({}) += {} * {}", output_digit, digit, pattern_element);
                output_digit += digit * pattern_element;
            }
            output_digit
        })
        .map(|digit| digit.abs() % 10)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod fft_phase_tests {
    use super::*;
    #[test]
    fn it_works() {
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result = fft_phase(&signal);
        assert_eq!(result, vec![4, 8, 2, 2, 6, 1, 5, 8]);
    }

    #[test]
    fn it_works_with_threads() {
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let start = 0;
        let len = 4;
        let result1 = fft_phase_in_parts(&signal, start, len);

        println!("done");
        let start = 4;
        /* len remains 4 */
        let result2 = fft_phase_in_parts(&signal, start, len);

        assert_eq!(result1, vec![4, 8, 2, 2]);
        assert_eq!(result2, vec![6, 1, 5, 8]);
    }

    #[test]
    fn it_works_with_threads_two() {
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let start = 0;
        let len = 3;
        let result1 = fft_phase_in_parts(&signal, start, len);

        println!("done");
        let start = 3;
        let len = 5;
        let result2 = fft_phase_in_parts(&signal, start, len);

        assert_eq!(result1, vec![4, 8, 2]);
        assert_eq!(result2, vec![2, 6, 1, 5, 8]);
    }
}

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

#[derive(Debug)]
struct RepeatingPatternIterator {
    /// The position in the signal. (0-based index)
    output_signal_idx: usize,
    /// The position in the base repeating pattern.
    pattern_position: usize,
    /// The number of times we have repeated the current number.
    repeat: usize,
}

impl RepeatingPatternIterator {
    fn new(output_signal_idx: usize, pattern_position: usize) -> Self {
        Self {
            output_signal_idx,
            pattern_position: pattern_position % BASE_PATTERN.len(),
            repeat: 0,
        }
    }
}

impl Iterator for RepeatingPatternIterator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.repeat += 1;
        if self.repeat > self.output_signal_idx {
            self.repeat = 0;
            self.pattern_position = (self.pattern_position + 1) % BASE_PATTERN.len();
        }
        Some(BASE_PATTERN[self.pattern_position])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // for the first digit in our signal, we should have a
    // repeating sequence of [1, 0, -1, 0, ...]
    #[test]
    fn first_digit() {
        let x = RepeatingPatternIterator::new(0, 0);
        let base_vals = x.take(5).collect::<Vec<_>>();
        assert_eq!(base_vals, vec![1, 0, -1, 0, 1]);
    }

    // for the third digit in our signal, we should have a
    // repeating sequence of [0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, ...]
    #[test]
    fn third_digit() {
        let x = RepeatingPatternIterator::new(2, 0);
        let base_vals = x.take(15).collect::<Vec<_>>();
        assert_eq!(
            base_vals,
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1]
        );
    }

    #[test]
    fn second_half() {
        let x = RepeatingPatternIterator::new(0, 2);
        let base_vals = x.take(5).collect::<Vec<_>>();
        assert_eq!(base_vals, vec![-1, 0, 1, 0, -1]);
    }

    #[test]
    fn try_example() {
        // For four phases of fft, assert that the output of each
        // phase is correct with respect to the expected value.
        let mut signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected_signal_outputs: Vec<Vec<i32>> = vec![
            vec![4, 8, 2, 2, 6, 1, 5, 8],
            vec![3, 4, 0, 4, 0, 4, 3, 8],
            vec![0, 3, 4, 1, 5, 5, 1, 8],
            vec![0, 1, 0, 2, 9, 4, 9, 8],
        ];

        for v in expected_signal_outputs.iter() {
            let _signal_output = fft_phase(&signal);
            assert_eq!(_signal_output, *v);
            signal = _signal_output;
        }
    }

    #[test]
    fn try_puzzle_input() {
        // For 100 phases of fft, assert that the output of the final
        // phase is the value that yields a gold star over at aoc.
        let s = fs::read_to_string("input.txt").unwrap();
        let s = s.trim();
        let mut signal_output;

        let mut signal_input = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();

        for _ in 0..100 {
            signal_output = fft_phase(&signal_input);
            signal_input = signal_output;
        }

        let first_eight = &signal_input[0..8];
        assert_eq!(first_eight, [4, 4, 0, 9, 8, 2, 6, 3]);
    }
}
