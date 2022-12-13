/*  https://adventofcode.com/2022/day/i  */

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
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_example()), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_example()), 0);
    }

    fn get_example() -> Input {
        Input::from_string(
            ""
        )
    }
}
