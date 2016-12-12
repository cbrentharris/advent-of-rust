#[derive(PartialEq, Debug)]
pub enum KeypadDirection {
    Up,
    Down,
    Left,
    Right
}

pub fn char_to_keypad_direction(keypad_char: char) -> KeypadDirection {
    match keypad_char {
        'U' => KeypadDirection::Up,
        'D' => KeypadDirection::Down,
        'L' => KeypadDirection::Left,
        'R' => KeypadDirection::Right,
        _ => panic!("Invalid keypad char: {}. Expected 'U', 'D', 'L', or 'R'", keypad_char)
    }
}

pub fn row_col_to_keypad_button(row: i32, col: i32) -> char {
    match (row, col) {
        (0, 0) => '1',
        (0, 1) => '2',
        (0, 2) => '3',
        (1, 0) => '4',
        (1, 1) => '5',
        (1, 2) => '6',
        (2, 0) => '7',
        (2, 1) => '8',
        (2, 2) => '9',
        (_, _) => panic!("Whoops!")
    }
}

pub fn day_2(input: String) {
    let mut row = 1;
    let mut col = 1;
    let MAX_ROW = 2;
    let MAX_COL = 2;
    let MIN_ROW = 0;
    let MIN_COL = 0;

    let split = input.split_whitespace().collect::<Vec<&str>>();
    for sequence in &split {
        for (i, c) in sequence.chars().enumerate() {
            let keypad_direction = char_to_keypad_direction(c);
            match keypad_direction {
                KeypadDirection::Up => {
                    if (row > MIN_ROW) {
                        row -= 1;
                    }
                },
                KeypadDirection::Down => {
                    if (row < MAX_ROW) {
                        row += 1;
                    }
                },
                KeypadDirection::Left => {
                    if (col > MIN_COL) {
                        col -= 1;
                    }
                },
                KeypadDirection::Right => {
                    if (col < MAX_COL) {
                        col += 1;
                    }
                }
            }
        }
    }
    row_col_to_keypad_button(row, col);
}
