
use std::fs;

fn main() {
    let input = fs::read_to_string("./2015/05/1/input.txt").expect("Unable to read file");
    println!("Answer is : {:?}", compute_solution(input));
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
    // It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.    
    let mut vowel_count = 0;
    for c in string.chars() {
        match &c.to_lowercase().to_string()[..] {
            "a" | "e" | "i" | "o" | "u" => vowel_count += 1,
            _ => continue,
        }
    }
    let rule1 = vowel_count >= 3;
    // It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    let mut rule2 = false;
    let mut prev = char::default();
    for c in string.chars() {
        if prev == c {
            rule2 = true;
            break;
        }
        prev = c;
    }

    ["ab", "cd", "pq", "xy"].iter().all(|s| !string.contains(s)) && rule1 && rule2
}
