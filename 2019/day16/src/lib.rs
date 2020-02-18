const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

struct BaseMask {
    /// The position in the signal.
    current_position: usize,
    /// The position in the base pattern.
    pattern_position: usize,
    /// The number of times we have repeated the current number.
    repeat: usize,
}

impl BaseMask {
    fn new(current_position: usize) -> Self {
        Self {
            current_position,
            pattern_position: 0,
            repeat: 0,
        }
    }
}

impl Iterator for BaseMask {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.repeat += 1;
        if self.repeat > self.current_position {
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
        let mut x = BaseMask::new(0);
        let base_vals = x.take(5).collect::<Vec<_>>();
        assert_eq!(base_vals, vec![1, 0, -1, 0, 1]);
    }

    // for the third digit in our signal, we should have a
    // repeating sequence of [0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, ...]
    #[test]
    fn third_digit() {
        let mut x = BaseMask::new(2);
        let base_vals = x.take(15).collect::<Vec<_>>();
        assert_eq!(
            base_vals,
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1]
        );
    }
}
