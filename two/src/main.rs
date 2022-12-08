use std::io::BufRead;
use std::{fs::File, io};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

fn your_move(their_move: Move, outcome_wanted: Outcome) -> Move {
    match outcome_wanted {
        Outcome::Draw => their_move,
        Outcome::Lose => match their_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        Outcome::Win => match their_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
    }
}

fn score(their_move: Move, your_move: Move) -> i32 {
    let your_shape_score = match &your_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    let outcome = match your_move {
        Move::Rock if their_move == Move::Scissors => 6,
        Move::Rock if their_move == Move::Paper => 0,

        Move::Paper if their_move == Move::Rock => 6,
        Move::Paper if their_move == Move::Scissors => 0,

        Move::Scissors if their_move == Move::Paper => 6,
        Move::Scissors if their_move == Move::Rock => 0,

        _ => 3,
    };

    your_shape_score + outcome
}

fn parse_move(s: &&str) -> Option<Move> {
    match *s {
        "A" => Some(Move::Rock),
        "B" => Some(Move::Paper),
        "C" => Some(Move::Scissors),
        _ => None,
    }
}

fn parse_outcome(s: &&str) -> Option<Outcome> {
    match *s {
        "X" => Some(Outcome::Lose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None,
    }
}

fn parse_line(line: String) -> Option<(Move, Outcome)> {
    let parts: Vec<&str> = line.split(' ').collect();
    let mv = parts.get(0).and_then(parse_move);
    let outcome = parts.get(1).and_then(parse_outcome);
    mv.zip(outcome)
}

fn main() {
    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let strings = lines.flat_map(|l| l.ok());
    let move_and_wanted_outcome = strings.flat_map(parse_line);

    let moves = move_and_wanted_outcome
        .map(|(their_move, wanted_outcome)| (their_move, your_move(their_move, wanted_outcome)));

    let scores = moves.map(|(them, you)| score(them, you));

    let answer: i32 = scores.sum();

    println!("The answer is {}", answer);
}
