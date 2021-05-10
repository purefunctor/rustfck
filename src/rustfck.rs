use std::collections::VecDeque;
use std::io::Read;
use std::num::Wrapping;
use Either::{Left, Right};
use Instruction::*;
use Token::*;

pub enum Token {
    MoveRight,
    MoveLeft,
    IncCell,
    DecCell,
    PrintIO,
    FetchIO,
    LoopStart,
    LoopStop,
}

pub enum Instruction {
    TapeRight,
    TapeLeft,
    CellInc,
    CellDec,
    CellPrint,
    CellFetch,
    TapeLoop(Vec<Instruction>),
}

// `Result` implies success and failure,
// `Either` is more generalized.
enum Either<L, R> {
    Left(L),
    Right(R),
}

pub fn read_token(c: char) -> Option<Token> {
    match c {
        '>' => Some(MoveRight),
        '<' => Some(MoveLeft),
        '+' => Some(IncCell),
        '-' => Some(DecCell),
        '.' => Some(PrintIO),
        ',' => Some(FetchIO),
        '[' => Some(LoopStart),
        ']' => Some(LoopStop),
        _ => None,
    }
}

pub fn make_instructions(tokens: Vec<Option<Token>>) -> Result<Vec<Instruction>, String> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut context: Vec<Vec<Instruction>> = Vec::new();

    for token in tokens {
        if let Some(t) = token {
            let e = match t {
                MoveRight => Right(TapeRight),
                MoveLeft => Right(TapeLeft),
                IncCell => Right(CellInc),
                DecCell => Right(CellDec),
                PrintIO => Right(CellPrint),
                FetchIO => Right(CellFetch),
                LoopStart => Left(LoopStart),
                LoopStop => Left(LoopStop),
            };
            if let Right(r) = e {
                if let Some(v) = context.last_mut() {
                    v.push(r);
                } else {
                    instructions.push(r);
                };
            } else if let Left(l) = e {
                match l {
                    LoopStart => {
                        context.push(Vec::new());
                    }
                    LoopStop => match (context.pop(), context.last_mut()) {
                        (Some(child), Some(parent)) => {
                            parent.push(TapeLoop(child));
                        }
                        (Some(child), None) => {
                            instructions.push(TapeLoop(child));
                        }
                        _ => {
                            return Err("unmatched loop".to_string());
                        }
                    },
                    _ => (),
                };
            };
        }
    }
    Ok(instructions)
}

type Cell = Wrapping<u8>;
type Tape = Vec<Cell>;

fn run(
    instructions: &Vec<Instruction>,
    tape: &mut Tape,
    pointer: &mut usize,
    input: &mut VecDeque<char>,
) -> Result<(), String> {
    for instruction in instructions {
        match instruction {
            TapeRight => {
                *pointer += 1;
            }
            TapeLeft => {
                *pointer -= 1;
            }
            CellInc => {
                tape.get_mut(*pointer).map(|x| *x += Wrapping(1));
            }
            CellDec => {
                tape.get_mut(*pointer).map(|x| *x -= Wrapping(1));
            }
            CellPrint => {
                print!("{}", tape.get_mut(*pointer).unwrap().0 as char);
            }
            CellFetch => {
                let mut i: [u8; 1] = [0; 1];
                match std::io::stdin().read_exact(&mut i) {
                    Ok(_) => {
                        tape.get_mut(*pointer).map(|_| Wrapping(i[0]));
                    }
                    Err(_) => {
                        return Err("cannot fetch".to_string());
                    }
                }
            }
            TapeLoop(i) => {
                while tape.get_mut(*pointer) != Some(&mut Wrapping(0)) {
                    if let Err(e) = run(i, tape, pointer, input) {
                        return Err(e);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn interpret(source: &str, buffer: &str) -> Result<(), String> {
    let instructions = make_instructions(source.chars().map(read_token).collect());

    let mut tape: Tape = vec![Wrapping(0); 1024];
    let mut pointer = 0;
    let mut input = buffer.chars().collect();

    instructions.and_then(|i| run(&i, &mut tape, &mut pointer, &mut input))
}
