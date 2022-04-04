use md5;

fn main() {
    println!("Answer for part1 is: {}", compute_solution("iwrupvqb", "00000"));
    println!("Answer for part2 is: {}", compute_solution("iwrupvqb", "000000"));
}

fn compute_solution(secret_key: &str, start_string: &str) -> u32 {
    let mut i = 0;
    loop {
        let output = md5::compute(&format!("{secret_key}{i}"));
        if format!("{:x}", output).starts_with(start_string) {
            break
        }
        i += 1;
    }
    i
}

