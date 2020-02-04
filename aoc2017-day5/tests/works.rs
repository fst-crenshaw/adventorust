use aoc2017_day5::foo;

#[test]
fn it_might_work() {
    assert_eq!(foo("tests/fixtures/smol-input.txt"), 5);
}

#[test]
fn left_bounds() {
    assert_eq!(foo("tests/fixtures/very-negative-number.txt"), 3);
}

#[test]
fn it_works() {
    assert_eq!(foo("tests/fixtures/input.txt"), 5);
}
