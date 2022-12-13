use std::collections::HashSet;
use std::io::BufRead;
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

fn run(instructions: Vec<Instruction>) {
    let mut cycle = 0;
    let mut x = 1;

    fn check_signal_strength(current_cycle: i32, current_x: i32) {
        if current_cycle % 40 == 0 {
            println!("");
        }

        let current_pixel = current_cycle % 40;

        let set = HashSet::from([current_x - 1, current_x, current_x + 1]);

        if set.contains(&current_pixel) {
    print!("#")
        }
        else {
            print!(".")
        }
    }

    for instruction in instructions {
        match instruction {
            Instruction::AddX(x_delta) => {
                check_signal_strength(cycle, x);
                cycle += 1;
                check_signal_strength(cycle, x);
                cycle += 1;
                x += x_delta;
            }
            Instruction::NoOp => {
                check_signal_strength(cycle, x);
                cycle += 1;
            }
        }
    }
    cycle += 1;
    check_signal_strength(cycle, x);

}

fn main() {
    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let strings = lines.flat_map(|l| l.ok());
    let instructions = strings.flat_map(parse_instruction);

    run(Vec::from_iter(instructions));
}
