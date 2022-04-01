use std::fs;

fn main() {
    let input = fs::read_to_string("./2015/01/1/input.txt").expect("Unable to read file");
    println!("Answer is : {:?}", compute_solution(input));
}

fn compute_solution(input: String) -> i32 {
    let mut i: i32 = 0;
    for c in input.chars() {
        match c {
            '(' => i += 1,
            ')' => i -= 1,
            _ => println!("unknown token: {:?}", c),
        } 
    }
    i
}
