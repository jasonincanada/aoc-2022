/*  https://adventofcode.com/2022/day/2  */

fn main() {
    let input = Input::from("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

enum Shape { Rock, Paper, Scissors }
enum Outcome { Win, Lose, Draw }

// so we don't have to keep writing Shape:: and Outcome:: in front of the enums
use crate::Shape::*;
use crate::Outcome::*;

// points we get for playing a certain shape
fn shape(shape: Shape) -> u32 {
    match shape {
        Rock     => 1,
        Paper    => 2,
        Scissors => 3
    }
}

// points we get for a certain outcome
fn outcome(outcome: Outcome) -> u32 {
    match outcome {
        Win  => 6,
        Draw => 3,
        Lose => 0
    }
}

fn letter_to_shape(letter: char) -> Shape {
    match letter {
        'A' | 'X' => Rock,
        'B' | 'Y' => Paper,
        'C' | 'Z' => Scissors,
         _        => panic!("Unexpected letter")
    }
}

fn letter_to_outcome(letter: char) -> Outcome {
    match letter {
        'X' => Lose,
        'Y' => Draw,
        'Z' => Win,
         _  => panic!("Unexpected letter")
    }
}

struct Round {
    them: Shape,

    // in part 1 this is the Shape we play
    // in part 2 it's the Outcome we want
    us  : char
}

fn points_for_part_1(round: &Round) -> u32 {
    match (&round.them, letter_to_shape(round.us)) {
        (Rock, Rock)         => shape(Rock)     + outcome(Draw),
        (Rock, Paper)        => shape(Paper)    + outcome(Win),
        (Rock, Scissors)     => shape(Scissors) + outcome(Lose),
        (Paper, Rock)        => shape(Rock)     + outcome(Lose),
        (Paper, Paper)       => shape(Paper)    + outcome(Draw),
        (Paper, Scissors)    => shape(Scissors) + outcome(Win),
        (Scissors, Rock)     => shape(Rock)     + outcome(Win),
        (Scissors, Paper)    => shape(Paper)    + outcome(Lose),
        (Scissors, Scissors) => shape(Scissors) + outcome(Draw),
    }
}

fn points_for_part_2(round: &Round) -> u32 {
    match (&round.them, letter_to_outcome(round.us)) {
        (Rock, Win)      => shape(Paper)    + outcome(Win),
        (Rock, Lose)     => shape(Scissors) + outcome(Lose),
        (Rock, Draw)     => shape(Rock)     + outcome(Draw),
        (Paper, Win)     => shape(Scissors) + outcome(Win),
        (Paper, Lose)    => shape(Rock)     + outcome(Lose),
        (Paper, Draw)    => shape(Paper)    + outcome(Draw),
        (Scissors, Win)  => shape(Rock)     + outcome(Win),
        (Scissors, Lose) => shape(Paper)    + outcome(Lose),
        (Scissors, Draw) => shape(Scissors) + outcome(Draw),
    }
}

struct Input {
    rounds: Vec<Round>
}

// the second letter on a line is the shape we play
fn part1(input: &Input) -> u32 {
    input.rounds.iter()
                .map(points_for_part_1)
                .sum()
}

// the second letter on a line is the outcome we want
fn part2(input: &Input) -> u32 {
    input.rounds.iter()
                .map(points_for_part_2)
                .sum()
}

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        let lines = contents.trim().split('\n');
        
        Input {
            rounds: lines.map(Input::line_to_round).collect()
        }
    }

    fn line_to_round(line: &str) -> Round {
        let mut letters = line.split_whitespace();
    
        Round {
            them: letter_to_shape(letters.next().unwrap().chars().next().unwrap()),
            us  :                 letters.next().unwrap().chars().next().unwrap()
        }
    }
}
