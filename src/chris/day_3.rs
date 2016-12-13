pub fn day_3(input: String) {
    let split = input.split("\n").collect::<Vec<&str>>();
    let mut possible_triangles = 0;
    for triangle in &split {
        let sides = triangle.split_whitespace().collect::<Vec<&str>>();
        let a = sides[0].parse::<i32>().unwrap();
        let b = sides[1].parse::<i32>().unwrap();
        let c = sides[2].parse::<i32>().unwrap();
        let a_is_possible = a < b + c;
        let b_is_possible = b < a + c;
        let c_is_possible = c < a + b;
        if a_is_possible && b_is_possible && c_is_possible {
            possible_triangles += 1;
        }
    }
    println!("{}", possible_triangles.to_string());
}
