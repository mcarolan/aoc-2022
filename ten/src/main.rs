use std::io::BufRead;
use std::str::SplitAsciiWhitespace;
use std::{fs::File, io};
enum Instruction {
    NoOp,
    AddX(i32),
}

struct State {
    cycle: u32,
    x: i32,
}

fn parse_instruction(line: String) -> Option<Instruction> {
    let parts = Vec::from_iter(line.split(' '));

    match parts.get(0) {
        Some(&"addx") => {
            let x_opt = parts.get(1).and_then(|s| i32::from_str_radix(s, 10).ok());
            x_opt.map(Instruction::AddX)
        }
        Some(&"noop") => Some(Instruction::NoOp),
        _ => None,
    }
}

fn run(instructions: Vec<Instruction>) -> i32 {
    let mut cycle = 0;
    let mut x = 1;

    let mut interesting_signal_strengths: Vec<i32> = Vec::new();

    fn check_signal_strength(current_cycle: i32, current_x: i32, buffer: &'_ mut Vec<i32>) {
        if current_cycle == 20 || (current_cycle + 20) % 40 == 0 {
            println!("cycle {} x {} - strength {}", current_cycle, current_x, current_cycle * current_x);
            buffer.push(current_cycle * current_x);
        }
    };

    for instruction in instructions {
        match instruction {
            Instruction::AddX(x_delta) => {
                cycle += 1;
                check_signal_strength(cycle, x, &mut interesting_signal_strengths);
                cycle += 1;
                check_signal_strength(cycle, x, &mut interesting_signal_strengths);
                x += x_delta;
            }
            Instruction::NoOp => {
                cycle += 1;
                check_signal_strength(cycle, x, &mut interesting_signal_strengths);
            }
        }
    }
    cycle += 1;
    check_signal_strength(cycle, x, &mut interesting_signal_strengths);

    interesting_signal_strengths.iter().sum()
}

fn main() {
    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let strings = lines.flat_map(|l| l.ok());
    let instructions = strings.flat_map(parse_instruction);

    let answer = run(Vec::from_iter(instructions));

    println!("The answer is {}", answer);
}
