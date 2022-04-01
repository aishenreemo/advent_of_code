
use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("./2015/03/1/input.txt").expect("Unable to read file");
    println!("Answer is : {:?}", compute_solution(input));
}

#[derive(Default, Eq, PartialEq, Clone, Hash)]
struct PersonPosition {
    column: i32,
    row: i32
}

fn compute_solution(input: String) -> usize {
    let mut cache: HashSet<PersonPosition> = HashSet::new();

    let mut santa = PersonPosition::default();
    let mut robot = PersonPosition::default();

    cache.insert(santa.clone());
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < input.len() {
        mutate_position(bytes.get(i), &mut santa);
        i += 1;
        mutate_position(bytes.get(i), &mut robot);
        i += 1;
        cache.insert(santa.clone());
        cache.insert(robot.clone());
    }
    cache.len()
}
fn mutate_position(c: Option<&u8>, person: &mut PersonPosition) {
    match *c.unwrap_or(&0) as char {
        '^' => person.row += 1,
        'v' => person.row -= 1,
        '>' => person.column += 1,
        '<' => person.column -= 1,
        _ => (),
    } 
}
