pub mod elves {
    // Given a string, determine whether it is nice.
    //    https://adventofcode.com/2015/day/5
    pub fn is_nice(s: &str) -> bool {
        let s = s.to_lowercase();
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let vowels = "aeiou";

        let mut vowel_count = 0;

        if is_naughty(&s) {
            return false;
        }

        /* Are there three vowels? */
        for v in vowels.chars() {
            for c in s.chars() {
                if v == c {
                    vowel_count = vowel_count + 1;
                }
            }
        }

        if vowel_count < 3 {
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
        false
    }

    fn is_naughty(s: &str) -> bool {
        match s {
            ns if ns.contains("ab") => true,
            ns if ns.contains("cd") => true,
            ns if ns.contains("pq") => true,
            ns if ns.contains("xy") => true,
            _ => false,
        }
    }

    fn test_is_nice(s: &str, expected: bool) -> bool {
        if is_nice(s) == expected {
            return true;
        }
        false
    }

    pub fn test_elves() {
        assert!(test_is_nice("ababababab", false));
        assert!(test_is_nice("baaaab", false));
        assert!(test_is_nice("ugknbfddgicrmopn", true));
        assert!(test_is_nice("aaa", true));
        assert!(test_is_nice("jchzalrnumimnmhp", false));
        assert!(test_is_nice("haegwjzuvuyypxyu", false));
        assert!(test_is_nice("dvszwmarrgswjxmb", false));
        assert!(test_is_nice("aalbblcclddee", true));
    }
}
