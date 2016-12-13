pub fn day_3(input: String) {
    let split = input.split("\n").collect::<Vec<&str>>();
    let mut possible_triangles = 0;
    let mut i = 0;
    while i < split.len() {
        let row_1 = split[i];
        let row_2 = split[i + 1];
        let row_3 = split[i + 2];

        let a_s = row_1.split_whitespace().collect::<Vec<&str>>();
        let b_s = row_2.split_whitespace().collect::<Vec<&str>>();
        let c_s = row_3.split_whitespace().collect::<Vec<&str>>();

        for j in 0..3 {
            let a = a_s[j].parse::<i32>().unwrap();
            let b = b_s[j].parse::<i32>().unwrap();
            let c = c_s[j].parse::<i32>().unwrap();
            let a_is_possible = a < b + c;
            let b_is_possible = b < a + c;
            let c_is_possible = c < a + b;
            if a_is_possible && b_is_possible && c_is_possible {
                possible_triangles += 1;
            }
        }
        i += 3;
    }
    println!("{}", possible_triangles.to_string());
}
