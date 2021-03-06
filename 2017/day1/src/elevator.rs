pub mod elevator {
    pub fn floor(choice: &str) -> i32 {
        let mut f = 0;

        for c in choice.chars() {
            match c {
                '(' => f = f + 1,
                ')' => f = f - 1,
                _ => return -1,
            }
        }
        f
    }

    fn test_floor(choice: &str, expected: i32) -> bool {
        if floor(choice) == expected {
            return true;
        }
        false
    }

    pub fn test_elevator() {
        assert!(test_floor("(())", 0));
        assert!(test_floor("(((", 3));
        assert!(test_floor("((()((()", 4));
        assert!(test_floor("(( ))", -1));
    }
}
