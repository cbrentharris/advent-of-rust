#[derive(PartialEq)]
#[derive(Debug)]
pub enum Direction {
  North,
  South,
  East,
  West
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TurnDirection {
  Left,
  Right
}

pub fn direction_now_facing(turn_direction: TurnDirection, currently_facing: Direction) -> Direction {
  match turn_direction {
    TurnDirection::Left => match currently_facing {
      Direction::North => Direction::West,
      Direction::South => Direction::East,
      Direction::East => Direction::North,
      Direction::West => Direction::South
    },
    TurnDirection::Right => match currently_facing {
      Direction::North => Direction::East,
      Direction::South => Direction::West,
      Direction::East => Direction::South,
      Direction::West => Direction::North
    }
  }
}

pub fn string_to_turn_direction(turn_str: &str) -> TurnDirection {
  match turn_str {
    "R" => TurnDirection::Right,
    "L" => TurnDirection::Left,
    _ => panic!("Invalid turn string: {}. Expected 'L' or 'R'" , turn_str)
  }
}
