
fn main() {
    let input = std::fs::read_to_string("2015/08/2/input.txt").expect("Unable to read file");
    println!("Answer is: {}", compute_solution(input));
}

fn compute_solution(input: String) -> u32 {
    let mut total_code_chars = 0;
    let mut total_encoded_chars = 0;

    for line in input.lines() {
        total_code_chars += line.len();
        total_encoded_chars += line.replace(r#"\"#, r#"\\"#).replace(r#"""#, r#"\""#).len() + 2;
    }
    (total_encoded_chars - total_code_chars) as u32
}
