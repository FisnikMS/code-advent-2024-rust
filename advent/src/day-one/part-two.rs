use std::{ fs, vec};
mod helper;

fn main() {
    let content = fs::read_to_string("files/day-2.txt").expect("Should have been able to read the file");
    let mut left_vec: Vec<i32> = vec![];
    let mut right_vec: Vec<i32> = vec![];

    for line in content.lines() {
        let tuple: (i32, i32) = match helper::split_to_tuple(line) {
            Ok((first, second)) => (first, second),
            Err(e) => panic!("{e}"),
        };
        
        left_vec.push(tuple.0);
        right_vec.push(tuple.1);
    }
    
    if left_vec.len() != right_vec.len() {
        panic!("argument size does not match");
    }

    let mut total_distance: i32 = 0;
    left_vec.iter().for_each(|num| {
        total_distance += num * helper::count_occurrences(&right_vec, *num); 
    });

    println!("The total distance is: {}", total_distance);

}
