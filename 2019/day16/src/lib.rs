#[allow(unused_imports)]
use std::fs;
use std::thread;

/// Perform a phase of our FFT process.
pub fn fft_phase(signal: &Vec<i32>) -> Vec<i32> {
    signal
        .iter()
        .enumerate()
        .map(|(i, _)| PatternMaker::new(i))
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
            let repeating_pattern = PatternMaker::new(i + start);
            let mut output_digit = 0;
            for (digit, pattern_element) in (*signal).iter().zip(repeating_pattern) {
                println!(
                    "output_digit ({}) += {} * {}",
                    output_digit, digit, pattern_element
                );
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
    fn it_works_in_halves() {
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let start = 0;
        let len = 4;
        let result1 = fft_phase_in_parts(&signal, start, len);

        let start = 4;
        /* len remains 4 */
        let result2 = fft_phase_in_parts(&signal, start, len);

        assert_eq!(result1, vec![4, 8, 2, 2]);
        assert_eq!(result2, vec![6, 1, 5, 8]);
    }

    #[test]
    fn it_works_lopsided() {
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let start = 0;
        let len = 3;
        let result1 = fft_phase_in_parts(&signal, start, len);

        let start = 3;
        let len = 5;
        let result2 = fft_phase_in_parts(&signal, start, len);

        assert_eq!(result1, vec![4, 8, 2]);
        assert_eq!(result2, vec![2, 6, 1, 5, 8]);
    }

    #[test]
    fn it_works_even_more_lopsided() {
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let start = 0;
        let len = 1;
        let result1 = fft_phase_in_parts(&signal, start, len);

        let start = 1;
        let len = 7;
        let result2 = fft_phase_in_parts(&signal, start, len);

        assert_eq!(result1, vec![4]);
        assert_eq!(result2, vec![8, 2, 2, 6, 1, 5, 8]);
    }
}

/// The PatternMaker type
///
const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

#[derive(Debug)]
struct PatternMaker {
    /// A pattern is determined by the desired index.  This index
    /// value describes how the pattern repeats itself.
    ///
    ///   If a calculation is being made for index 0, then the pattern
    ///   begins (1 0 -1) and then repeats (0 1 0 -1) thereafter.
    ///
    ///   If a calculation is being made for index 1, then the pattern begins
    ///   (0 1 1 0 0 -1 -1) and then repeats (0 0 1 1 0 0 -1 -1) thereafter.
    index: usize,

    /// The total length of the pattern that repeats.  For example, an
    /// index 0 pattern has length 4, an index 1 pattern has length 8,
    /// an index 2 pattern has length 12, and so on.
    pattern_length: usize,

    /// For implementing an iterator over the pattern, the pattern
    /// position indicates the position of the pattern that was last
    /// returned.  For a pattern of index 2, the pattern begins
    /// 0 0 1 1 1 0 0 0 -1 -1 -1. If the pattern position is 5,
    /// then the value last returned by an iterator was:
    ///
    ///  0 0 1 1 1 0 0 0 -1 -1 -1
    ///            ^
    ///            |----- This value, at position 5.
    pattern_position: usize,
}

impl PatternMaker {
    fn new(index: usize) -> Self {
        Self {
            index,
            pattern_length: (index + 1) * 4,
            pattern_position: 0,
        }
    }
}

impl Iterator for PatternMaker {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        // The BASE_PATTERN is four values: 0 1 0 -1.  We need to
        // calculate an index of 0, 1, 2 or 3 into the BASE_PATTERN.
        let base_pattern_index =
            ((self.pattern_position + 1) % self.pattern_length) / (self.index + 1);

        // Keep track of where we are in the pattern by incrementing
        // the position by 1.
        self.pattern_position = self.pattern_position + 1;
        Some(BASE_PATTERN[base_pattern_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // for the first digit in our signal, we should have a
    // repeating sequence of [1, 0, -1, 0, ...]
    #[test]
    fn first_digit() {
        let x = PatternMaker::new(0);
        let base_vals = x.take(5).collect::<Vec<_>>();
        assert_eq!(base_vals, vec![1, 0, -1, 0, 1]);
    }

    // for the third digit in our signal, we should have a
    // repeating sequence of [0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, ...]
    #[test]
    fn third_digit() {
        let x = PatternMaker::new(2);
        let base_vals = x.take(15).collect::<Vec<_>>();
        assert_eq!(
            base_vals,
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1]
        );
    }

    #[test]
    fn second_half() {
        let x = PatternMaker::new(2);
        let base_vals = x.take(5).collect::<Vec<_>>();
        assert_eq!(base_vals, vec![0, 0, 1, 1, 1,]);
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
    fn try_example_in_parts() {
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
            let _signal_output_front = fft_phase_in_parts(&signal, 0, 4);
            let _signal_output_back = fft_phase_in_parts(&signal, 4, 4);
            let mut _signal_output = _signal_output_front;
            _signal_output.extend(_signal_output_back);
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

    #[test]
    fn try_puzzle_input_in_parts() {
        // For 100 phases of fft, assert that the output of the final
        // phase is the value that yields a gold star over at aoc.
        let s = fs::read_to_string("input.txt").unwrap();
        let s = s.trim();
        let mut signal_output_front;
        let mut signal_output_back;

        let mut signal_input = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();

        // How big is the input?
        let half = signal_input.len() / 2;

        for _ in 0..100 {
            signal_output_front = fft_phase_in_parts(&signal_input, 0, half);
            signal_output_back = fft_phase_in_parts(&signal_input, half, half * 2);
            signal_output_front.extend(signal_output_back);
            signal_input = signal_output_front;
        }

        let first_eight = &signal_input[0..8];
        assert_eq!(first_eight, [4, 4, 0, 9, 8, 2, 6, 3]);
    }
}
