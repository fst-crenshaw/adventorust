pub mod elves {

    pub struct Present(u32, u32, u32);

    // Given a Christmas Present's dimensions, determine how
    // many square feet of wrapping paper are necessary for wrapping.
    //    https://adventofcode.com/2015/day/2
    pub fn get_paper(p: &Present) -> u32 {
        let Present(l, w, h) = p;

        let dims = [l * w, w * h, h * l];

        let min_dim = dims.iter().fold(std::u32::MAX, |mut min_val, d| {
            min_val = std::cmp::min(min_val, *d);
            min_val
        });

        (2 * l * w + 2 * w * h + 2 * h * l) + min_dim
    }

    // Given a string, determine whether it is nice.
    //    https://adventofcode.com/2015/day/5
    pub fn is_nice(s: &str) -> bool {
        let s = s.to_lowercase();
        let alphabet = "abcdefghijklmnopqstuvwxyz";
        let vowels = "aeiou";

        if is_naughty(&s) {
            return false;
        }

        /* Are there any doubles? */
        for c in alphabet.chars() {
            let dub = c.to_string();
            let dub = dub.repeat(2);

            if s.contains(&dub) {
                return true;
            }
        }

        /* Are there three vowels? */
        let mut vowel_count = 0;
        for v in vowels.chars() {
            let v = v.to_string();

            if s.contains(&v) {
                vowel_count += 1;
            }
        }
        if vowel_count == 3 {
            return true;
        }
        false
    }

    fn is_naughty(s: &str) -> bool {
        match s {
            ns if ns.contains("ab") => true,
            ns if ns.contains("cd") => true,
            ns if ns.contains("pq") => true,
            ns if ns.contains("xy") => true,
            _ => true,
        }
    }

    // *** Tests
    fn test_get_paper(p: &Present, expected: u32) -> bool {
        if get_paper(p) == expected {
            return true;
        }
        false
    }

    fn test_is_nice(s: &str, expected: bool) -> bool {
        if is_nice(s) == expected {
            return true;
        }
        false
    }

    pub fn test_elves() {
        assert!(test_get_paper(&Present(2, 3, 4), 58));
        assert!(test_get_paper(&Present(1, 1, 10), 43));

        assert!(test_is_nice("ababababab", false));
        assert!(test_is_nice("baaaab", false));
    }
}
