use std::collections::HashMap;
use regex::Regex;
use std::iter::FromIterator;


pub fn day_4(input: String) {
    let rooms = input.split("\n").collect::<Vec<&str>>();
    let mut valid_rooms = 0;
    for room in rooms {
        let parts = room.split("-").collect::<Vec<&str>>();
        let mut count: HashMap<char, u32> = HashMap::new();
        for i in 0..parts.len() - 1 {
            let part = parts[i];
            for c in part.chars() {
                *count.entry(c).or_insert(0) += 1;
            }
        }
        let mut count_vec: Vec<(&char, &u32)> = count.iter().collect();
        count_vec.sort_by(|a, b| b.0.cmp(a.0));
        count_vec.reverse();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        let top_five: String = String::from_iter(
            count_vec
                .iter()
                .map(|item| *item.0)
                .take(5)
        );

        let re = Regex::new(r"\[|\]").unwrap();
        // e.g. 123[abc]
        let mut last_part_split = re.split(parts[parts.len() - 1]);
        let room_number = last_part_split.next().unwrap();
        let checksum = last_part_split.next().unwrap();
        if top_five.to_string() == checksum {
            valid_rooms += room_number.parse::<i32>().unwrap();
        }
    }
    println!("{}", valid_rooms);
}
