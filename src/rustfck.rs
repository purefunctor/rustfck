use either::Either::{Left, Right};
use std::io::Read;
use std::num::Wrapping;
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

impl Token {
    pub fn from_char(c: char) -> Option<Self> {
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

impl Instruction {
    fn from_tokens(tokens: Vec<Token>) -> Result<Vec<Self>, String> {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut context: Vec<Vec<Instruction>> = Vec::new();

        for token in tokens.into_iter() {
            let e = match token {
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
                context.last_mut().unwrap_or(&mut instructions).push(r);
            } else if let Left(l) = e {
                match l {
                    LoopStart => {
                        context.push(Vec::new());
                    }
                    LoopStop => {
                        if let Some(child) = context.pop() {
                            context
                                .last_mut()
                                .unwrap_or(&mut instructions)
                                .push(TapeLoop(child));
                        } else {
                            return Err("unmatched brackets!".to_string());
                        }
                    }
                    _ => (),
                };
            };
        }
        Ok(instructions)
    }
}

type Cell = Wrapping<u8>;
type Tape = Vec<Cell>;

pub struct Interpreter {
    instructions: Vec<Instruction>,
    tape: Tape,
    pointer: usize,
}

impl Interpreter {
    pub fn from_source(source: &str) -> Result<Self, String> {
        Instruction::from_tokens(
            source
                .chars()
                .map(Token::from_char)
                .flat_map(|t| t.into_iter())
                .collect(),
        )
        .map(|i| Interpreter {
            instructions: i,
            tape: vec![Wrapping(0); 30000],
            pointer: 0,
        })
    }

    fn run_(
        instructions: &[Instruction],
        tape: &mut Tape,
        pointer: &mut usize,
    ) -> Result<(), String> {
        for instruction in instructions {
            match instruction {
                TapeRight => *pointer += 1,
                TapeLeft => *pointer -= 1,
                CellInc => {
                    if let Some(c) = tape.get_mut(*pointer) {
                        *c += Wrapping(1);
                    }
                }
                CellDec => {
                    if let Some(c) = tape.get_mut(*pointer) {
                        *c -= Wrapping(1);
                    }
                }
                CellPrint => {
                    if let Some(c) = tape.get_mut(*pointer) {
                        print!("{}", c.0 as char);
                    }
                }
                CellFetch => {
                    let mut i: [u8; 1] = [0; 1];
                    match std::io::stdin().read_exact(&mut i) {
                        Ok(_) => {
                            if let Some(c) = tape.get_mut(*pointer) {
                                *c = Wrapping(i[0]);
                            }
                        }
                        Err(_) => {
                            return Err("cannot fetch".to_string());
                        }
                    }
                }
                TapeLoop(i) => {
                    while let Some(n) = tape.get(*pointer) {
                        if n.0 == 0 {
                            break;
                        }
                        if let Err(e) = Interpreter::run_(i, tape, pointer) {
                            return Err(e);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        Interpreter::run_(&self.instructions, &mut self.tape, &mut self.pointer)
    }
}
