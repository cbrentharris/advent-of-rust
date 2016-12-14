extern crate advent_lib;
use advent_lib::chris::day_4::{day_4};
use advent_lib::input_reader;

#[test]
fn day_4_test() {
    let mut s = String::new();
    input_reader::read_file("src/resources/day4/chris_input.txt", &mut s);
    day_4(s);
    // Uncomment to see output of day_4
    // assert_eq!(true, false);
}
