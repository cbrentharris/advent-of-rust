extern crate advent_lib;
use advent_lib::input_reader;
use advent_lib::ian::day1;

fn main() {
  let mut directions = String::new();
  input_reader::read_file("src/resources/day1/ian_input.txt", &mut directions);
  day1::find_twice_visited_place(directions);
}
