use std::fs;

fn main() {
    let input = fs::read_to_string("./2015/01/2/input.txt").expect("Unable to read file");
    println!("Answer is: {:?}", compute_solution(input));
}

fn compute_solution(input: String) -> i32 {
    let mut i: i32 = 0;
    for (j, c) in input.chars().enumerate() {
        match c {
            '(' => i += 1,
            ')' => i -= 1,
            '\n' => continue,
            _ => println!("unknown token: {:?}", c),
        } 
        if i < 0 {
            return j as i32 + 1;
        }
    }
    i
}
