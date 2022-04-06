fn main() {
    let input = std::fs::read_to_string("2015/08/1/input.txt").expect("Unable to read file");
    println!("Answer is: {}", compute_solution(input));
}

fn compute_solution(input: String) -> u32 {
    let mut total_code_chars = 0;
    let mut total_string_chars = 0;

    for line in input.lines() {
        let mut cursor = 1;
        let mut chars_len = 0;
        let chars: Vec<char> = line.chars().collect();
        while cursor < line.len() - 1 {
            match chars.get(cursor) {
                Some(&'"') => unreachable!("quote inside quote"),
                Some(&'\\') => expect_escape_char(&mut cursor, &mut chars_len, &chars),
                Some(_) => chars_len += 1,
                _ => unreachable!("expected char"),
            }
            cursor += 1;
        }
        cursor += 1;
        total_code_chars += cursor;
        total_string_chars += chars_len;
    }
    (total_code_chars - total_string_chars) as u32
}

fn expect_escape_char(cursor: &mut usize, chars_len: &mut usize, chars: &[char]) {
    *cursor += 1;
    match chars.get(*cursor) {
        Some(&'x') => *cursor += 2,
        Some(_) => (),
        None => unreachable!("expected escape char"),
    }
    *chars_len += 1;
}
