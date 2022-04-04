use std::fs;

fn main() {
    let input = fs::read_to_string("./2015/05/2/input.txt").expect("Unable to read file");
    println!("Answer is: {:?}", compute_solution(input));
}

fn compute_solution(input: String) -> usize {
    let mut i = 0;
    for line in input.lines() {
        if is_string_nice(line) {
            i += 1;
        }
    }
    i
}

fn is_string_nice(string: &str) -> bool {
    let mut rule1 = false;
    let mut rule2 = false;

    let vector = string.chars().collect::<Vec<char>>();
    let mut cursor = 0;

    while cursor < string.len() - 1 {
        let mut vec = string.chars().collect::<Vec<char>>();
        let substring: String = format!("{}{}", vec[cursor], vec[cursor + 1]); 
        vec[cursor] = '_';
        vec[cursor + 1] = '_';
        if vec.iter().collect::<String>().contains(&substring) {
            rule1 = true;
            break;
        }
        cursor += 1;
    }
    for current in vector.windows(3) {
        if current[0] == current[2] {
            rule2 = true;
            break;
        }
    }

    rule1 && rule2
}

