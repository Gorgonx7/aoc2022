use std::fs;
const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSS: i32 = 0;

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";
const NEED_TO_LOSE: &str = "X";
const NEED_TO_DRAW: &str = "Y";
const NEED_TO_WIN: &str = "Z";
const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Unable to open");
    let split = contents.split("\n");
    let mut result: i32 = 0;
    for s in split {
        let split_letters = s.split(" ");
        let array_letters: Vec<&str> = split_letters.collect();
        // now we figure out who won
        // println!("{}", array_letters[1]);
        let opponent_plays = array_letters[0];
        let i_play = array_letters[1];
        match opponent_plays {
            OPPONENT_ROCK => result += rock_vs(i_play),
            OPPONENT_PAPER => result += paper_vs(i_play),
            OPPONENT_SCISSORS => result += scissors_vs(i_play),
            _ => println!("invalid: {}", opponent_plays),
        }
    }
    println!("Result: {}", result);
}

fn rock_vs(other: &str) -> i32 {
    match other {
        NEED_TO_LOSE => return LOSS + SCISSORS,
        NEED_TO_DRAW => return DRAW + ROCK,
        NEED_TO_WIN => return WIN + PAPER,
        _ => return 0,
    }
}
fn paper_vs(other: &str) -> i32 {
    match other {
        NEED_TO_LOSE => return LOSS + ROCK,
        NEED_TO_DRAW => return DRAW + PAPER,
        NEED_TO_WIN => return WIN + SCISSORS,
        _ => return 0,
    }
}
fn scissors_vs(other: &str) -> i32 {
    match other {
        NEED_TO_LOSE => return LOSS + PAPER,
        NEED_TO_DRAW => return DRAW + SCISSORS,
        NEED_TO_WIN => return WIN + ROCK,
        _ => return 0,
    }
}
