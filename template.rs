/*  https://adventofcode.com/2022/day/4  */

fn main() {
    let input = Input::from("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { field : Vec<u32> }

//
fn part1(input: &Input) -> u32 {
    0
}

//
fn part2(input: &Input) -> u32 {
    0
}

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        let lines = contents.trim().split('\n');
        
        Input {
            field: vec![]
        }
    }    
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {        
        assert_eq!(1, 1);
    }
}
