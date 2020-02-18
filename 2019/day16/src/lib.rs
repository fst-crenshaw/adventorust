/// Perform a phase of our FFT process.
pub fn fft_phase(signal: Vec<i32>) -> Vec<i32> {
    signal
        .iter()
        .enumerate()
        .map(|(i, _)| RepeatingPatternIterator::new(i))
        .map(|repeating_pattern| {
            let mut output_digit = 0;
            for (digit, pattern_element) in signal.iter().zip(repeating_pattern) {
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
        let result = fft_phase(signal);
        assert_eq!(result, vec![4, 8, 2, 2, 6, 1, 5, 8]);
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
    fn new(output_signal_idx: usize) -> Self {
        Self {
            output_signal_idx,
            pattern_position: 0,
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
            self.pattern_position = (self.pattern_position + 1) % 4;
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
        let x = RepeatingPatternIterator::new(0);
        let base_vals = x.take(5).collect::<Vec<_>>();
        assert_eq!(base_vals, vec![1, 0, -1, 0, 1]);
    }

    // for the third digit in our signal, we should have a
    // repeating sequence of [0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, ...]
    #[test]
    fn third_digit() {
        let x = RepeatingPatternIterator::new(2);
        let base_vals = x.take(15).collect::<Vec<_>>();
        assert_eq!(
            base_vals,
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1]
        );
    }
}
