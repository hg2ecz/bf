use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Clone)]
enum OpCode {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}

fn char_to_opcode(source: String) -> Vec<OpCode> {
    let mut op = Vec::new();
    for ch in source.chars() {
        match ch {
            '>' => op.push(OpCode::IncrementPointer),
            '<' => op.push(OpCode::DecrementPointer),
            '+' => op.push(OpCode::Increment),
            '-' => op.push(OpCode::Decrement),
            '.' => op.push(OpCode::Write),
            ',' => op.push(OpCode::Read),
            '[' => op.push(OpCode::LoopBegin),
            ']' => op.push(OpCode::LoopEnd),
            _ => (),
        };
    }
    op
}

#[derive(Clone)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

fn parse_loops(opcodes: Vec<OpCode>) -> Vec<Instruction> {
    let mut prog: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_begin = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            match op {
                OpCode::IncrementPointer => prog.push(Instruction::IncrementPointer),
                OpCode::DecrementPointer => prog.push(Instruction::DecrementPointer),
                OpCode::Increment => prog.push(Instruction::Increment),
                OpCode::Decrement => prog.push(Instruction::Decrement),
                OpCode::Write => prog.push(Instruction::Write),
                OpCode::Read => prog.push(Instruction::Read),

                OpCode::LoopBegin => {
                    loop_begin = i;
                    loop_stack += 1;
                }

                OpCode::LoopEnd => panic!("loop ending at #{} has no beginning", i),
            };
        } else {
            match op {
                OpCode::LoopBegin => {
                    loop_stack += 1;
                }
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        let instr_vec = parse_loops(opcodes[loop_begin + 1..i].to_vec());
                        prog.push(Instruction::Loop(instr_vec));
                    }
                }
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        panic!(
            "loop that starts at #{} has no matching ending!",
            loop_begin
        );
    }

    prog
}

fn run_program(instructions: &[Instruction], tape: &mut Vec<u8>, data_pointer: &mut usize) {
    for instr in instructions {
        match instr {
            Instruction::IncrementPointer => *data_pointer += 1,
            Instruction::DecrementPointer => *data_pointer -= 1,
            Instruction::Increment => tape[*data_pointer] += 1,
            Instruction::Decrement => tape[*data_pointer] -= 1,
            Instruction::Write => print!("{}", tape[*data_pointer] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("failed to read stdin");
                tape[*data_pointer] = input[0];
            }
            Instruction::Loop(nested_instructions) => {
                while tape[*data_pointer] != 0 {
                    run_program(&nested_instructions, tape, data_pointer)
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: bf <file.bf>");
        std::process::exit(1);
    }
    let mut file = File::open(&args[1]).expect("program file not found");

    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("failed to read program file");

    let opcodes = char_to_opcode(source);
    let program = parse_loops(opcodes);

    let mut tape: Vec<u8> = vec![0; 1024];
    let mut data_pointer = 512;
    run_program(&program, &mut tape, &mut data_pointer);
}
