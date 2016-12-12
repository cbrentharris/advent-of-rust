extern crate advent_lib;
use advent_lib::chris::day_2::{KeypadDirection, char_to_keypad_direction, day_2};
use advent_lib::input_reader;

#[test]
fn keypad_char_to_direction() {
    assert_eq!(char_to_keypad_direction('L'), KeypadDirection::Left);
    assert_eq!(char_to_keypad_direction('R'), KeypadDirection::Right);
    assert_eq!(char_to_keypad_direction('U'), KeypadDirection::Up);
    assert_eq!(char_to_keypad_direction('D'), KeypadDirection::Down);
}

#[test]
fn day_1_test() {
    let mut s = String::new();
    input_reader::read_file("src/resources/day2/chris_input.txt", &mut s);
    day_2(s);
    // Uncomment to see output of day_2
    // assert_eq!(true, false);
}
