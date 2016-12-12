use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub enum Direction {
  North,
  South,
  East,
  West
}

#[derive(PartialEq, Debug)]
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

pub fn string_to_turn_direction(turn_str: char) -> TurnDirection {
  match turn_str {
    'R' => TurnDirection::Right,
    'L' => TurnDirection::Left,
    _ => panic!("Invalid turn string: {}. Expected 'L' or 'R'" , turn_str)
  }
}

pub fn check_point(x: i32, y: i32, set: &mut HashSet<(i32, i32)>) -> bool {
  println!("Checking {}, {}", x.to_string(), y.to_string());
  if set.contains(&(x, y)) {
    println!("First duplicate coordinate: {}, {}. It is {} blocks away.",
             x.to_string(),
             y.to_string(),
             (x.abs() + y.abs()).to_string()
    );
    return true;
  } else {
    set.insert((x, y));
    return false;
  }
}

pub fn day_1(input: String) {
  let mut x = 0;
  let mut y = 0;
  let split = input.split(", ").collect::<Vec<&str>>();
  let mut current_direction = Direction::North;
  let mut coord_set: HashSet<(i32, i32)> = HashSet::new();
  let mut found_visited_point = false;
  for direction in &split {
    let direction_string = String::from(*direction);
    let mut direction_chars = direction_string.chars();
    let turn_direction = string_to_turn_direction(direction_chars.next().unwrap());
    let remaining = direction_chars.as_str().trim();
    let num_steps = remaining
        .parse::<i32>().unwrap();
    current_direction = direction_now_facing(turn_direction, current_direction);
    match current_direction {
      Direction::North => {
        for i in y..y + num_steps {
          if !found_visited_point {
            found_visited_point = check_point(x, i, &mut coord_set);
          }
        }
        y += num_steps;
      },
      Direction::South => {
        for i in (y - (num_steps - 1)..y + 1).rev() {
          if !found_visited_point {
            found_visited_point = check_point(x, i, &mut coord_set);
          }
        }
        y -= num_steps
      },
      Direction::East =>{
        for i in x..x + num_steps {
          if !found_visited_point {
            found_visited_point = check_point(i, y, &mut coord_set);
          }
        }
        x += num_steps
      },
      Direction::West => {
        for i in (x - (num_steps - 1)..x + 1).rev() {
          if !found_visited_point {
            found_visited_point = check_point(i, y, &mut coord_set);
          }
        }
        x -= num_steps
      }
    }
  }
  println!("{}", (y.abs() + x.abs()).to_string());
}

