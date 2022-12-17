/*  https://adventofcode.com/2022/day/1  */

fn main() {
    let input = get_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { elves : Vec<Elf> }
struct Elf   { cals  : Vec<u32> }

impl Input {
    fn from(input: &str) -> Self {
        let chunks = input.trim().split("\n\n");

        Input {
            elves: chunks.map(Elf::from).collect()
        }
    }
}

impl Elf {
    fn from(chunk: &str) -> Self {
        Elf {
            cals: chunk.split('\n')
                       .map(|line| line.parse().unwrap())
                       .collect()
        }
    }

    fn get_total_cals(&self) -> u32 {
        self.cals.iter().sum()
    }
}

// get the highest total calorie count
fn part1(input: &Input) -> u32 {
    sum_highest(1, input)    
}

// sum the 3 highest total calorie counts
fn part2(input: &Input) -> u32 {
    sum_highest(3, input)    
}

fn sum_highest(count: usize, input: &Input) -> u32 {
    let mut totals: Vec<u32> =
        input.elves.iter()
                   .map(|elf| elf.get_total_cals())
                   .collect();

    totals.sort();
    totals.iter()
          .rev()
          .take(count)
          .sum()
}

fn get_input() -> Input {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input.txt");
    
    Input::from(&input)
}

/*
    $ cargo run
    Compiling aoc-2022 v0.1.0 (C:\Users\Jason\Documents\GitHub\aoc-2022\day_01)
        Finished dev [unoptimized + debuginfo] target(s) in 0.42s
        Running `target\debug\aoc-2022.exe`
    Part 1: 69626
    Part 2: 206780
*/
