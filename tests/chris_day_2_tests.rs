extern crate advent_lib;
use advent_lib::chris::day_2::{KeypadDirection, char_to_keypad_direction};
use advent_lib::input_reader;

#[test]
fn turn_string_to_turn_direction_test() {
    assert_eq!(char_to_keypad_direction('L'), KeypadDirection::Left);
    assert_eq!(char_to_keypad_direction('R'), KeypadDirection::Right);
    assert_eq!(char_to_keypad_direction('U'), KeypadDirection::Up);
    assert_eq!(char_to_keypad_direction('D'), KeypadDirection::Down);
}
