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
        _ => panic!("Invalid turn string: {}. Expected 'U', 'D', 'L', or 'R'", keypad_char)
    }
}
