pub fn drop_smallest_entry(vec: &mut Vec<i32>) -> i32 {
    let tuple: (usize, i32) = match get_min(vec) {
        Some((index, number)) => (index, number),
        None => panic!("Something went wrong."),
    }; 
    vec.remove(tuple.0);
    return tuple.1;
}

pub fn split_to_tuple(input: &str) -> Result<(i32,i32), String> {
    let mut parts = input.split_whitespace();
    let first: String = match parts.next() {
        Some(part) => part.to_string(),
        None => panic!("First argument is not an integer")
    };

    let second: String = match parts.next() {
        Some(part) => part.to_string(),
        None => panic!("Second argument is not an integer")
    };
    if parts.next().is_none() {
        return create_tuple(&first, &second);
    } else {
        return Err(String::from("Line has too many arguments."));
    }
}

pub fn count_occurrences(vec: &Vec<i32>, num: i32) -> i32 {
    return vec.iter().filter(|entry: &&i32| **entry == num).collect::<Vec<_>>().len() as i32;
}

fn get_min(vec: &Vec<i32>) -> Option<(usize, i32)> {
    return vec 
       .iter()
       .enumerate()
       .min_by_key(|&(_, value)| value)
       .map(|(index, &value)| (index, value));

}

fn create_tuple(first: &str, second: &str) -> Result<(i32,i32), String> {
    let first_integer: i32 = match parse_integer(first) {
        Ok(integer) => integer,
        Err(e) => panic!("{e}")
    };

    let second_integer: i32 = match parse_integer(second) {
        Ok(integer) => integer,
        Err(e) => panic!("{e}")
    };
    return Ok((first_integer, second_integer));
}

fn parse_integer(input: &str) -> Result<i32, String> {
    return input.parse::<i32>().map_err(|e| e.to_string());
}
