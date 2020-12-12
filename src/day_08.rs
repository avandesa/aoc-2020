use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

impl OpCode {
    pub fn from_str(input: &str) -> Self {
        match input {
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            "nop" => Self::Nop,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: OpCode,
    arg: i32,
}

impl Instruction {
    pub fn parse(input: &str) -> Self {
        let (raw_opcode, raw_arg) = input.split(' ').collect_tuple().unwrap();

        Self {
            opcode: OpCode::from_str(raw_opcode),
            arg: raw_arg.parse().unwrap(),
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let instructions = input.lines().map(Instruction::parse).collect::<Vec<_>>();
    let mut executed = HashSet::<usize>::new();
    let mut instr_ptr = 0;
    let mut acc = 0i32;

    loop {
        if executed.contains(&instr_ptr) {
            break acc;
        }
        executed.insert(instr_ptr);

        let instruction = &instructions[instr_ptr];

        instr_ptr = match instruction.opcode {
            OpCode::Nop => instr_ptr + 1,
            OpCode::Acc => {
                acc += instruction.arg;
                instr_ptr + 1
            }
            OpCode::Jmp => instr_ptr.wrapping_add(instruction.arg as usize),
        }
    }
}

pub fn part2(_input: &str) -> i32 {
    todo!()
}
