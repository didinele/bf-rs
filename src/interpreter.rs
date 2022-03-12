use crate::parser::*;
use std::io::Read;

#[derive(Debug)]
pub struct Interpreter {
    memory: Vec<u8>,
    ptr: usize,
    idx: usize,
    instructions: Vec<Instruction>,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>) -> Interpreter {
        Interpreter {
            memory: vec![0],
            ptr: 0,
            idx: 0,
            instructions,
        }
    }

    fn run_instruction(&mut self, instruction: Instruction) -> () {
        match instruction.kind {
            InstructionKind::DecrementPtr => {
                assert!(self.ptr != 0, "Tried to decrement while ptr was at 0");
                self.ptr -= 1;
            }

            InstructionKind::IncrementPtr => {
                if self.ptr + 1 >= self.memory.len() {
                    self.memory.push(0);
                }

                self.ptr += 1;
            }

            InstructionKind::Decrement => {
                debug_assert!(
                    self.ptr <= self.memory.len(),
                    "Tried to decrement while ptr was out of bounds"
                );
                self.memory[self.ptr] -= 1;
            }

            InstructionKind::Increment => {
                debug_assert!(
                    self.ptr <= self.memory.len(),
                    "Tried to increment while ptr was out of bounds"
                );
                self.memory[self.ptr] += 1;
            }

            InstructionKind::Output => {
                debug_assert!(
                    self.ptr <= self.memory.len(),
                    "Tried to output while ptr was out of bounds"
                );
                print!("{}", self.memory[self.ptr] as char);
            }

            InstructionKind::Input => {
                debug_assert!(
                    self.ptr <= self.memory.len(),
                    "Tried to input while ptr was out of bounds"
                );
                self.memory[self.ptr] = std::io::stdin().bytes().nth(0).unwrap().unwrap();
            }

            InstructionKind::LoopBegin => {
                debug_assert!(
                    self.ptr <= self.memory.len(),
                    "Tried to loop while ptr was out of bounds"
                );
                if self.memory[self.ptr] == 0 {
                    let mut depth = 1;
                    let mut j = self.idx + 1;

                    while depth != 0 {
                        match self.instructions[j].kind {
                            InstructionKind::LoopBegin => depth += 1,
                            InstructionKind::LoopEnd => depth -= 1,
                            _ => (),
                        }

                        j += 1;
                    }

                    self.idx = j - 1;
                }
            }

            InstructionKind::LoopEnd => {
                debug_assert!(
                    self.ptr <= self.memory.len(),
                    "Tried to end loop while ptr was out of bounds"
                );
                if self.memory[self.ptr] != 0 {
                    let mut depth = 1;
                    let mut j = self.idx - 1;

                    while depth != 0 {
                        match self.instructions[j].kind {
                            InstructionKind::LoopBegin => depth -= 1,
                            InstructionKind::LoopEnd => depth += 1,
                            _ => (),
                        }

                        j -= 1;
                    }

                    self.idx = j + 1;
                }
            }
        }
    }

    pub fn run(&mut self) -> () {
        loop {
            let instruction = self.instructions[self.idx];
            self.run_instruction(instruction);
            self.idx += 1;

            if self.idx == self.instructions.len() {
                break;
            }
        }
    }
}
