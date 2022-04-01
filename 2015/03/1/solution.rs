use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("./2015/03/1/input.txt").expect("Unable to read file");
    println!("Answer is : {:?}", compute_solution(input));
}

fn compute_solution(input: String) -> usize {
    let mut cache: HashSet<(i32, i32)> = HashSet::new();
    // santa position
    let mut column: i32 = 0;
    let mut row: i32 = 0;
    
    cache.insert((column, row));
    for c in input.chars() {
        match c {
            '^' => row += 1,
            'v' => row -= 1,
            '>' => column += 1,
            '<' => column -= 1,
            _ => continue,
        }
        cache.insert((column, row));
    }
    cache.len()
}
