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

/**
   1
  2 3 4
5 6 7 8 9
  A B C
    D
*/
pub fn row_col_to_keypad_button(row: i32, col: i32) -> char {
    match (row, col) {
        (0, 2) => '1',
        (1, 1) => '2',
        (1, 2) => '3',
        (1, 3) => '4',
        (2, 0) => '5',
        (2, 1) => '6',
        (2, 2) => '7',
        (2, 3) => '8',
        (2, 4) => '9',
        (3, 1) => 'A',
        (3, 2) => 'B',
        (3, 3) => 'C',
        (4, 2) => 'D',
        (_, _) => panic!("Whoops!")
    }
}

/**
* This function says if you can move up on a diagonal based keypad based on your current row and col
*/
pub fn can_move_up(row: i32, col: i32, max_row: i32, max_col: i32) -> bool {
    let mid = (max_row / 2 - col).abs();
    return row - mid > 0;
}

pub fn can_move_down(row: i32, col: i32, max_row: i32, max_col: i32) -> bool {
    let mid = (max_row / 2 - col).abs();
    return row + mid < max_row;
}

pub fn can_move_left(row: i32, col: i32, max_row: i32, max_col: i32) -> bool {
    let mid = (max_row / 2 - row).abs();
    return col - mid > 0;
}

pub fn can_move_right(row: i32, col: i32, max_row: i32, max_col: i32) -> bool {
    let mid = (max_row / 2 - row).abs();
    return col + mid < max_col;
}

pub fn day_2(input: String) {
    let mut row = 3;
    let mut col = 0;
    let MAX_ROW = 4;
    let MAX_COL = 4;
    let MIN_ROW = 0;
    let MIN_COL = 0;

    let split = input.split_whitespace().collect::<Vec<&str>>();
    for sequence in &split {
        for (i, c) in sequence.chars().enumerate() {
            let keypad_direction = char_to_keypad_direction(c);
            match keypad_direction {
                KeypadDirection::Up => {
                    if can_move_up(row, col, MAX_ROW, MAX_COL) {
                        row -= 1;
                    }
                },
                KeypadDirection::Down => {
                    if can_move_down(row, col, MAX_ROW, MAX_COL) {
                        row += 1;
                    }
                },
                KeypadDirection::Left => {
                    if can_move_left(row, col, MAX_ROW, MAX_COL) {
                        col -= 1;
                    }
                },
                KeypadDirection::Right => {
                    if can_move_right(row, col, MAX_ROW, MAX_COL) {
                        col += 1;
                    }
                }
            }
        }
        print!("{}", row_col_to_keypad_button(row, col));
    }
}
