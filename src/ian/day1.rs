struct Location {
  facing: i32,
  x: i32,
  y: i32
}

impl Location {
  fn movement(&mut self, direction: String) {
    let turn = direction.chars().next();
    let steps_string = &direction[1..direction.len()];
    let steps = steps_string.parse::<i32> ().unwrap();

    match turn {
      Some('L') => self.facing -= 1,
      Some('R') => self.facing += 1,
      Some(_) | None => println!("Weird: Invalid Direction")
    }

    match self.facing {
      1 | 5 => {
        self.facing = 1;
        self.y += steps;
      },
      2 => self.x += steps,
      3 => self.y -= steps,
      0 | 4 => {
        self.facing = 4;
        self.x -= steps;
      },
      _ => panic!("Facing an Invalid Direction")
    };
  }
}

pub fn find_distance(direction_string: String) {
  let mut start = Location { x: 0, y: 0, facing: 1 };
  let location = direction_string.split(", ").fold(&mut start, |position, direction| {
    position.movement(String::from(direction));
    return position;
  });
  println!("{}", location.x.abs() + location.y.abs());
}