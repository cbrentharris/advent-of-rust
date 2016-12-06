extern crate advent_lib;
use advent_lib::chris::day_1::{Direction, TurnDirection, direction_now_facing};
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
