/*  https://adventofcode.com/2022/day/10  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}",  part1(&input));
    println!("Part 2:\n{}", part2(&input));
}

struct Input { instructions: Vec<Instruction> }

enum Instruction {
    NoOp,
    AddX(i32)
}

use Instruction::*;

// calculate the value of the register at each cycle
fn part1(input: &Input) -> i32 {
    let mut register = 1;
    let mut cycles: Vec<i32> = vec![register];

    for instruction in input.instructions.iter() {
        match instruction {

            NoOp => {
                cycles.push(register);
            },

            AddX(x) => {
                cycles.push(register);
                cycles.push(register);

                // update the register only at the end of the cycle
                register += x;
            }
        }
    }

    let get_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];

    get_cycles.into_iter()
              .map(|cycle| cycle * cycles[cycle as usize])
              .sum()
}

// draw an ASCII diagram on a CRT
fn part2(input: &Input) -> String {
    let mut register: i32 = 1;
    let mut cycle: i32 = 1;
    let mut crt: String = String::new();

    for instruction in input.instructions.iter() {
        match instruction {

            NoOp => {
                output_to_crt(&mut crt, register, cycle); cycle += 1;
            },

            AddX(x) => {
                output_to_crt(&mut crt, register, cycle); cycle += 1;
                output_to_crt(&mut crt, register, cycle); cycle += 1;

                // update the register only at the end of the cycle
                register += x;
            }
        }
    }

    crt
}

fn output_to_crt(crt: &mut String, register: i32, cycle: i32) {

    // wrap to 40 columns
    let cycle = (cycle-1) % 40;

    if [cycle-1, cycle, cycle+1].contains(&register) {
        crt.push('#');
    } else {
        crt.push('.');
    }

    if cycle+1 == 40 { crt.push('\n'); }
}


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            instructions: s.lines()
                           .map(Instruction::from_line)
                           .collect()
        }
    }
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        match line {
            "noop" => NoOp,
                 _ => AddX(line.split(' ')
                               .nth(1).unwrap()
                               .parse().unwrap())
        }
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_example()), 13140);
    }

    #[test]
    fn test_part2() {
        let expected = "##..##..##..##..##..##..##..##..##..##..\n\
                        ###...###...###...###...###...###...###.\n\
                        ####....####....####....####....####....\n\
                        #####.....#####.....#####.....#####.....\n\
                        ######......######......######......####\n\
                        #######.......#######.......#######.....\n".to_string();

        assert_eq!(part2(&get_example()), expected);
    }

    fn get_example() -> Input {
        Input::from("example.txt")
    }
}

/*  $ cargo run

    Part 1: 17840
    Part 2:
    ####..##..#.....##..#..#.#....###...##..
    #....#..#.#....#..#.#..#.#....#..#.#..#.
    ###..#..#.#....#....#..#.#....#..#.#....
    #....####.#....#.##.#..#.#....###..#.##.
    #....#..#.#....#..#.#..#.#....#....#..#.
    ####.#..#.####..###..##..####.#.....###.
*/
