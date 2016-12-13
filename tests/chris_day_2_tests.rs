extern crate advent_lib;
use advent_lib::chris::day_2::{KeypadDirection, char_to_keypad_direction, can_move_up, can_move_down, can_move_left, can_move_right, day_2};
use advent_lib::input_reader;

#[test]
fn keypad_char_to_direction() {
    assert_eq!(char_to_keypad_direction('L'), KeypadDirection::Left);
    assert_eq!(char_to_keypad_direction('R'), KeypadDirection::Right);
    assert_eq!(char_to_keypad_direction('U'), KeypadDirection::Up);
    assert_eq!(char_to_keypad_direction('D'), KeypadDirection::Down);
}

#[test]
fn can_move_up_test() {
    let max_row = 4;
    let max_col = 4;
    assert!(!can_move_up(2, 0, max_row, max_col));
    assert!(!can_move_up(1, 1, max_row, max_col));
    assert!(!can_move_up(0, 2, max_row, max_col));
    assert!(!can_move_up(1, 3, max_row, max_col));
    assert!(!can_move_up(2, 4, max_row, max_col));


    assert!(can_move_up(2, 1, max_row, max_col));
    assert!(can_move_up(3, 1, max_row, max_col));

    assert!(can_move_up(1, 2, max_row, max_col));
    assert!(can_move_up(2, 2, max_row, max_col));
    assert!(can_move_up(3, 2, max_row, max_col));
    assert!(can_move_up(4, 2, max_row, max_col));

    assert!(can_move_up(2, 3, max_row, max_col));
    assert!(can_move_up(3, 3, max_row, max_col));
}

#[test]
fn can_move_down_test() {
    let max_row = 4;
    let max_col = 4;
    assert!(!can_move_down(2, 0, max_row, max_col));
    assert!(!can_move_down(3, 1, max_row, max_col));
    assert!(!can_move_down(4, 2, max_row, max_col));
    assert!(!can_move_down(3, 3, max_row, max_col));
    assert!(!can_move_down(2, 4, max_row, max_col));


    assert!(can_move_down(1, 1, max_row, max_col));
    assert!(can_move_down(2, 1, max_row, max_col));

    assert!(can_move_down(0, 2, max_row, max_col));
    assert!(can_move_down(1, 2, max_row, max_col));
    assert!(can_move_down(2, 2, max_row, max_col));
    assert!(can_move_down(3, 2, max_row, max_col));

    assert!(can_move_down(1, 3, max_row, max_col));
    assert!(can_move_down(2, 3, max_row, max_col));
}

#[test]
fn can_move_left_test() {
    let max_row = 4;
    let max_col= 4;
    assert!(!can_move_left(2, 0, max_row, max_col));
    assert!(!can_move_left(1, 1, max_row, max_col));
    assert!(!can_move_left(0, 2, max_row, max_col));
    assert!(!can_move_left(3, 1, max_row, max_col));
    assert!(!can_move_left(4, 2, max_row, max_col));

    assert!(can_move_left(1, 2, max_row, max_col));
    assert!(can_move_left(1, 3, max_row, max_col));

    assert!(can_move_left(2, 1, max_row, max_col));
    assert!(can_move_left(2, 2, max_row, max_col));
    assert!(can_move_left(2, 3, max_row, max_col));
    assert!(can_move_left(2, 4, max_row, max_col));

    assert!(can_move_left(3, 2, max_row, max_col));
    assert!(can_move_left(3, 3, max_row, max_col));
}

#[test]
fn can_move_right_test() {
    let max_row = 4;
    let max_col= 4;
    assert!(!can_move_right(0, 2, max_row, max_col));
    assert!(!can_move_right(1, 3, max_row, max_col));
    assert!(!can_move_right(2, 4, max_row, max_col));
    assert!(!can_move_right(3, 3, max_row, max_col));
    assert!(!can_move_right(4, 2, max_row, max_col));

    assert!(can_move_right(1, 1, max_row, max_col));
    assert!(can_move_right(1, 2, max_row, max_col));

    assert!(can_move_right(2, 0, max_row, max_col));
    assert!(can_move_right(2, 1, max_row, max_col));
    assert!(can_move_right(2, 2, max_row, max_col));
    assert!(can_move_right(2, 3, max_row, max_col));

    assert!(can_move_right(3, 1, max_row, max_col));
    assert!(can_move_right(3, 2, max_row, max_col));
}

#[test]
fn day_2_test() {
    let mut s = String::new();
    input_reader::read_file("src/resources/day2/chris_input.txt", &mut s);
    day_2(s);
    // Uncomment to see output of day_2
    // assert_eq!(true, false);
}
