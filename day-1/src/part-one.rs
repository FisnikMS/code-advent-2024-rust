use std::{ fs, vec};
mod helper;

fn main() {
    let content = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
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
    
    let mut index: usize = 0; 
    let rounds: usize = left_vec.len();
    let mut total_distance: i32 = 0;

    while index < rounds {
        let first_min = helper::drop_smallest_entry(&mut left_vec);
        let second_min = helper::drop_smallest_entry(&mut right_vec);
        total_distance += (first_min - second_min).abs();
        index = index + 1;
    }

    println!("The total distance is: {}", total_distance);

}

