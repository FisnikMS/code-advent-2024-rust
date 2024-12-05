use std::fs;

#[derive(PartialEq)]
enum Direction {
    INCREASING,
    DECREASING,
    STATELESS
}

impl Direction {
    pub fn compute(&self, last_level: i32, level: i32) -> bool {
        return match self {
            Direction::INCREASING => last_level < level,
            Direction::DECREASING => last_level > level,
            Direction::STATELESS => panic!("Compute shouldn't be called on stateless")
        } && (last_level - level).abs() > 0 && (last_level - level).abs() < 4
    }
}

fn main() {
    let content = fs::read_to_string("files/day-2.txt").expect("Should have been able to read the file");

    let safe_report : Vec<Vec<&str>> = content
    .lines()
    .map(|line: &str| line.trim().split_whitespace().collect())
    .filter(|report: &Vec<&str>| validate_report_with_toleration(report))
    .collect::<Vec<_>>();

    print!("{} reports are safe", safe_report.len());
}

fn validate_report_with_toleration(report: &Vec<&str>) -> bool {
    let is_valid = validate_report(report); 
    
    if !is_valid {
        for i in 0..report.len() {
            let slice: Vec<&str> = [&report[0..i], &report[i+1..]].concat();  
            if validate_report(&slice) {
                return true;
            }
        }
    }
    return is_valid;
}

fn validate_report(report: &Vec<&str>) -> bool {
    let mut direction: Direction = Direction::STATELESS;
    let res : Option<&[&str]>= report 
        .windows(2)
        .find(|level_pair: &&[&str]| !validate_pairs(level_pair[0],level_pair[1], &mut direction));
    
    return res.is_none();    
}

fn validate_pairs(last_level: &str, level: &str, direction: &mut Direction) -> bool {
    let last_level_int: i32 = last_level.parse::<i32>().expect("can't parse last level");
    let level_int: i32 = level.parse::<i32>().expect("can't parse level");

    if *direction == Direction::STATELESS {
        *direction = if last_level_int < level_int {Direction::INCREASING} else {Direction::DECREASING};
    }

    return direction.compute(last_level_int, level_int);
}

