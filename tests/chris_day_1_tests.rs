extern crate advent_lib;
use advent_lib::chris::day_1::{Direction, TurnDirection, direction_now_facing, string_to_turn_direction, day_1};
use advent_lib::input_reader;

#[test]
fn direction_facing_test() {
  assert_eq!(direction_now_facing(TurnDirection::Left, Direction::South), Direction::East);
  assert_eq!(direction_now_facing(TurnDirection::Left, Direction::North), Direction::West);
  assert_eq!(direction_now_facing(TurnDirection::Left, Direction::West), Direction::South);
  assert_eq!(direction_now_facing(TurnDirection::Left, Direction::East), Direction::North);

  assert_eq!(direction_now_facing(TurnDirection::Right, Direction::South), Direction::West);
  assert_eq!(direction_now_facing(TurnDirection::Right, Direction::North), Direction::East);
  assert_eq!(direction_now_facing(TurnDirection::Right, Direction::West), Direction::North);
  assert_eq!(direction_now_facing(TurnDirection::Right, Direction::East), Direction::South);
}

#[test]
fn turn_string_to_turn_direction_test() {
  assert_eq!(string_to_turn_direction('L'), TurnDirection::Left);
  assert_eq!(string_to_turn_direction('R'), TurnDirection::Right);
}

#[test]
fn day_1_test() {
  let mut s = String::new();
  input_reader::read_file("src/resources/day1/chris_input.txt", &mut s);
  day_1(s);
  // Uncomment to see output of day_1
  // assert_eq!(true, false);
}
