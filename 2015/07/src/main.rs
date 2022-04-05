use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token{
    pub eval_expression: Expression,
    pub output_indentifier: String,
}

#[derive(Debug, Clone)]
pub enum Indentifier {
    Variable(String),
    Integer(u16),
}

#[derive(Debug, Clone)]
pub enum Expression {
    And(Indentifier, Indentifier),
    Or(Indentifier, Indentifier),
    LShift(Indentifier, u16),
    RShift(Indentifier, u16),
    Not(Indentifier),
    Literal(Indentifier),
}

fn main() {
    // get the input to evaluate
    let input = std::fs::read_to_string("input.txt").expect("Unable to read file.");

    // transform the input string into a chunck of tokens to make it easy to evaluate later
    let tokens = tokenize(&input);
    
    // part 1 is getting the value of a
    let part1 = compute_solution(&tokens, HashMap::new());
    println!("part1 answer: {:#?}", part1.get("a"));

    // part 2 is overwrite a wire "b" with a new value from part1, reset all wires except "b" and find the new value of a
    let part2 = compute_solution(&tokens, HashMap::from([("b".to_owned(), *part1.get("a").unwrap())]));
    println!("part2 answer: {:#?}", part2.get("a"));
}

fn compute_solution(tokens: &[Token], mut memory: HashMap<String, u16>) -> HashMap<String, u16> {
    use Expression::*;
    let mut pointer = 0;
    let mut amount_of_successfully_evaluated_tokens = 0;
    loop {
        let token = tokens.get(pointer).unwrap();

        let evaluated_output: u16 = match token.eval_expression.clone() {
            And(i1, i2) => {
                if [&i1, &i2].iter().any(|i| into_u16(&memory, *i).is_none()) {
                    mutate_pointer(&mut pointer, tokens.len());
                    amount_of_successfully_evaluated_tokens = 0;
                    continue;
                }
                *into_u16(&memory, &i1).unwrap() & *into_u16(&memory, &i2).unwrap()
            },
            Or(i1, i2) => {
                if [&i1, &i2].iter().any(|i| into_u16(&memory, *i).is_none()) {
                    mutate_pointer(&mut pointer, tokens.len());
                    amount_of_successfully_evaluated_tokens = 0;
                    continue;
                }
                *into_u16(&memory, &i1).unwrap() | *into_u16(&memory, &i2).unwrap()
            },
            LShift(i, n) => {
                if into_u16(&memory, &i).is_none(){
                    mutate_pointer(&mut pointer, tokens.len());
                    amount_of_successfully_evaluated_tokens = 0;
                    continue;
                }
                *into_u16(&memory, &i).unwrap() << n
            },
            RShift(i, n) => {
                if into_u16(&memory, &i).is_none(){
                    mutate_pointer(&mut pointer, tokens.len());
                    amount_of_successfully_evaluated_tokens = 0;
                    continue;
                }
                *into_u16(&memory, &i).unwrap() >> n
            },
            Not(i) => {
                if into_u16(&memory, &i).is_none() {
                    mutate_pointer(&mut pointer, tokens.len());
                    amount_of_successfully_evaluated_tokens = 0;
                    continue;
                }
                !*into_u16(&memory, &i).unwrap()
            },
            Literal(i) => {
                if into_u16(&memory, &i).is_none() {
                    mutate_pointer(&mut pointer, tokens.len());
                    amount_of_successfully_evaluated_tokens = 0;
                    continue;
                }
                *into_u16(&memory, &i).unwrap()
            },
        };

        amount_of_successfully_evaluated_tokens += 1;
        if !memory.contains_key(&token.output_indentifier) {
            memory.insert(token.output_indentifier.clone(), evaluated_output); 
        }
        mutate_pointer(&mut pointer, tokens.len());

        if amount_of_successfully_evaluated_tokens == tokens.len() {
            break
        }
    }
    memory
}

fn mutate_pointer(pointer: &mut usize, maximum_limit: usize) {
    *pointer = (*pointer + 1) % maximum_limit; 
}

fn into_u16<'a>(memory: &'a HashMap<String, u16>, i: &'a Indentifier) -> Option<&'a u16> {
    use Indentifier::*;
    match i {
        Variable(s) => memory.get(s),
        Integer(num) => Some(num),
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut cursor = 0;
    let chars: Vec<char> = input.chars().collect();
   
    while !is_tokenizer_done(cursor, &chars) {
        use Expression::*;
        
        // yield a list of argruments from a statement then split with "->"
        // e.g.
        // a AND b -> c
        // [a, AND, b], [c]
        let iter = expect_statement(&mut cursor, &chars);
        let mut iter = iter.split(|s| &s[..] == "->");

        // convert pattern into a Token structure
        let eval_expression: Vec<&str> = iter.next().unwrap().iter().map(|x| &**x).collect();
        let eval_expression = match &eval_expression[..] {
            [i] => Literal(expect_identifier(i)),
            [i1, "OR", i2] => Or(expect_identifier(i1), expect_identifier(i2)),
            [i1, "AND", i2] => And(expect_identifier(i1), expect_identifier(i2)),
            [i1, "LSHIFT", i2] => LShift(expect_identifier(i1), i2.parse::<u16>().unwrap()),
            [i1, "RSHIFT", i2] => RShift(expect_identifier(i1), i2.parse::<u16>().unwrap()),
            ["NOT", i] => Not(expect_identifier(i)),
            _ => unreachable!(),
        };

        tokens.push(Token {
            eval_expression,
            output_indentifier: iter.next().unwrap()[0].clone(),
        })
    }
    tokens 
}

fn expect_identifier(identifier: &str) -> Indentifier {
    use Indentifier::*; 
    match identifier {
        // if the indentifier string is numeric (e.g. 123)
        i if i.chars().all(|n| n.is_numeric()) => Integer(i.parse::<u16>().unwrap()),
        // if the indentifier string is alphabetic and lowercased (e.g. abc)
        i if i.chars().all(|c| matches!(c, 'a'..='z')) => Variable(i.to_string()),
        _ => unreachable!(),
    }
}

fn expect_statement(cursor: &mut usize, chars: &[char]) -> Vec<String> {
    let mut args = vec![];
    let mut temp_arg = "".to_owned();
    while !is_tokenizer_done(*cursor, chars) {
        match chars.get(*cursor) {
            Some(&'\n') => break,
            Some(' ') => {
                args.push(temp_arg.clone());
                temp_arg = "".to_owned();
            },
            Some(c) => temp_arg.push(*c),
            _ => unreachable!(),
        }
        *cursor += 1;
    }
    args.push(temp_arg);
    *cursor += 1;
    args
}

fn is_tokenizer_done(cursor: usize, chars: &[char]) -> bool {
    cursor >= chars.len()
}
