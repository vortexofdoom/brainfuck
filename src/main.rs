#[derive(Debug)]
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

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::MoveRight(n) => write!(f, "(Move right {n})"),
            Operation::MoveLeft(n) => write!(f, "(Move left {n})"),
            Operation::Increment(n) => write!(f, "(Increment {n})"),
            Operation::Decrement(n) => write!(f, "(Decrement {n})"),
            Operation::Output => write!(f, "Output"),
            Operation::Input => write!(f, "Input"),
            Operation::Open(n) => write!(f, "(Start loop: end at {n})"),
            Operation::Close(n) => write!(f, "(End loop: start at {n})"),
        }
    }
}

fn main() {
    for op in parse(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.").iter() {
        println!("{}", format!("{op}"));
    }
    //brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]);
}

// fn ez_vec(s: &str, i: u8) -> Vec<u8> {
//     let mut v = s.to_string().into_bytes();
//     v.push(i);
//     v
// }   

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut res = vec![];
    let mut tape: Vec<u8> = vec![0; 40000];
    let ops = parse(code);
    let mut i = 0;
    let mut cell: usize = 0;
    let mut input_i = 0;
    
    while i < ops.len() {
        match ops[i] {
            Operation::MoveRight(n) => cell += n,
            Operation::MoveLeft(n) => cell -= n,
            Operation::Increment(n) => tape[cell] = tape[cell].wrapping_add(n),
            Operation::Decrement(n) => tape[cell] = tape[cell].wrapping_sub(n),
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
    let mut brackets: Vec<usize> = vec![];
    let mut bracket_i = 0;
    let mut offset = 0;

    for i in 0..instructions.len() {
        match instructions[i] {
            '>' => {
                if let Some(Operation::MoveRight(x)) = operations.last_mut() {
                    *x += 1;
                    offset += 1;
                } else {
                    operations.push(Operation::MoveRight(1));
                }
            }
            '<' => {
                if let Some(Operation::MoveLeft(x)) = operations.last_mut() {
                    *x += 1;
                    offset += 1;
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
                brackets.push(i - offset);
                operations.push(Operation::Open(bracket_i));
                bracket_i += 1;
            },
            ']' => {
                if let Some(e) = brackets.get(bracket_i - 1) {
                    operations.remove(*e);
                    operations.insert(*e, Operation::Open(i));
                    operations.push(Operation::Close(*e));
                    bracket_i -= 1;
                }
                brackets.remove(bracket_i);
            },
            _ => {}
        }
    }
    operations
}