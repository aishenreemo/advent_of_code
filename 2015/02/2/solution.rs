
use std::fs;

fn main() {
    let input = fs::read_to_string("./2015/02/2/input.txt").expect("Unable to read file");
    println!("Answer is : {:?}", compute_solution(input));
}

fn compute_solution(input: String) -> i32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        match &line.split('x').collect::<Vec<&str>>()[..] {
            [a, b, c] => total += calculate_sqr_feet(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap(), c.parse::<u32>().unwrap()),
            _ => continue,
        }
    }
    total as i32
}

fn calculate_sqr_feet(l: u32, w: u32, h: u32) -> u32 {
    let mut min_nums = vec![l, w, h];
    min_nums.sort();
    (l * w * h) + 2 * (min_nums[0] + min_nums[1])
}
