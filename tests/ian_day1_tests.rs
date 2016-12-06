extern crate advent_lib;
use advent_lib::ian;

#[test]
fn day1() {
  let mut directions = String::new();
  input_reader::read_file("src/resources/day1/ian_input.txt", &mut directions);

  find_distance(directions);
}
