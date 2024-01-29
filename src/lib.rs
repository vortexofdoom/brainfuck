enum Operation {
    MoveRight(usize),
    MoveLeft(usize),
    Increment(u8),
    Decrement(u8),
    Output,
    Input,
    Open(usize),
    Close(usize),
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut res = vec![];
    let mut tape: Vec<u8> = vec![0; 40000];
    let ops = parse(code);
    let mut i = 0;
    let mut cell = 0;
    let mut input_i = 0;
    
    while i < ops.len() {
        match ops[i] {
            Operation::MoveRight(n) => cell += n,
            Operation::MoveLeft(n) => cell -= n,
            Operation::Increment(n) => tape[cell] += n,
            Operation::Decrement(n) => tape[cell] -= n,
            Operation::Output => res.push(tape[cell]),
            Operation::Input => {
                tape[cell] = input[input_i];
                input_i += 1;
            },
            Operation::Open(u) => {
                if tape[cell] == 0 {
                    i = u;
                }
            },
            Operation::Close(u) => {
                if tape[cell] != 0 {
                    i = u;
                }
            },
        }
        i += 1;
    }
    res
}

fn parse(code: &str) -> Vec<Operation> {
    let instructions: Vec<char> = code
        .chars()
        .filter(|c| match *c {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
            _ => false
        })
        .collect();
    
    let mut operations:Vec<Operation> = vec![];
    let mut bracket_i = 0;
    let mut brackets: Vec<(usize, usize)> = vec![];

    for i in 0..instructions.len() {
        match instructions[i] {
            '>' => {
                if let Some(Operation::MoveRight(x)) = operations.last_mut() {
                    *x += 1;
                } else {
                    operations.push(Operation::MoveRight(1));
                }
            }
            '<' => {
                if let Some(Operation::MoveLeft(x)) = operations.last_mut() {
                    *x += 1;
                } else {
                    operations.push(Operation::MoveLeft(1));
                }
            }
            '+' => {
                if let Some(Operation::Increment(x)) = operations.last_mut() {
                    *x += 1;
                } else {
                    operations.push(Operation::Increment(1));
                }
            }
            '-' => {
                if let Some(Operation::Decrement(x)) = operations.last_mut() {
                    *x += 1;
                } else {
                    operations.push(Operation::Decrement(1));
                }
            }
            '.' => operations.push(Operation::Output),
            ',' => operations.push(Operation::Input),
            '[' => {
                brackets.push((i, 0));
                operations.push(Operation::Open(bracket_i));
                bracket_i += 1;
            },
            ']' => {
                if let Some(mut e) = brackets.get_mut(bracket_i) {
                    e.1 = i;
                    operations.push(Operation::Close(bracket_i));
                    bracket_i -= 1;
                }
            },
            _ => {}
        }
    }
    operations
        .into_iter()
        .enumerate()
        .map(|(i, o)| {
            match o {
                Operation::Open(n) => {
                    if let Some(e) = brackets.get(n) {
                        Operation::Open(e.1)
                    } else {
                        o
                    }
                }
                Operation::Close(n) => {
                    if let Some(e) = brackets.get(n) {
                        Operation::Close(e.0)
                    } else {
                        o
                    }
                },
                _ => o
            }
        })
        .collect()
}