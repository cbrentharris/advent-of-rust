extern crate advent_lib;
use advent_lib::chris::day_3::{day_3};
use advent_lib::input_reader;

#[test]
fn day_3_test() {
    let mut s = String::new();
    input_reader::read_file("src/resources/day3/chris_input.txt", &mut s);
    day_3(s);
    // Uncomment to see output of day_1
    // assert_eq!(true, false);
}
