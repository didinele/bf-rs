#[derive(Debug, Clone, Copy)]
pub enum InstructionKind {
    IncrementPtr,
    DecrementPtr,
    Increment,
    Decrement,
    Output,
    Input,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub index: usize,
    pub ch: char,
}

impl Instruction {
    pub fn from(ch: char, index: usize) -> Option<Instruction> {
        let kind = match ch {
            '>' => InstructionKind::IncrementPtr,
            '<' => InstructionKind::DecrementPtr,
            '+' => InstructionKind::Increment,
            '-' => InstructionKind::Decrement,
            '.' => InstructionKind::Output,
            ',' => InstructionKind::Input,
            '[' => InstructionKind::LoopBegin,
            ']' => InstructionKind::LoopEnd,
            _ => return None,
        };

        Some(Instruction { kind, index, ch })
    }
}

pub fn parse_tokens(program: String) -> Vec<Instruction> {
    let mut tokens: Vec<Instruction> = Vec::new();

    for (i, c) in program.chars().enumerate() {
        if let Some(instruction) = Instruction::from(c, i) {
            tokens.push(instruction);
        }
    }

    tokens
}
