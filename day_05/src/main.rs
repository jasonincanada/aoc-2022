/*  https://adventofcode.com/2022/day/5  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input {
    ship : Ship,
    moves: Vec<Move>
}

type Ship  = Vec<Vec<Crate>>;
type Crate = char;

struct Move {
    amount: usize,
    from  : usize,
    to    : usize
}

// move 1 crate at a time
fn part1(input: &Input) -> String {

    // clone the ship from the input since we'll need to mutate it when moving crates around
    let mut ship = input.ship.clone();

    for m in &input.moves {
        for _ in 0 .. m.amount {
            let letter = ship[m.from].pop().unwrap();
            ship[m.to].push(letter);
        }
    }

    top_crates(&ship)
}

// a move of more than 1 crate takes all of them at once (preserves their order)
fn part2(input: &Input) -> String {
    let mut ship = input.ship.clone();

    for m in &input.moves {
        let     at   = ship[m.from].len() - m.amount;
        let mut grab = ship[m.from].split_off(at);     // rust's built-in Vec::split_off()

        ship[m.to].append(&mut grab);
    }

    top_crates(&ship)
}

// build a string from the top crate from every stack that has a crate
fn top_crates(ship: &Ship) -> String {   
    ship.iter()
        .skip(1)
        .filter_map(|stack| stack.iter().last())
        .collect()
}


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");

        // the input is in two sections, the ship graphic and the move list
        let (ship, moves) = contents.trim().split_once("\n\n").unwrap();
        
        Input {
            ship : Input::parse_ship(ship),
            moves: Input::parse_moves(moves)
        }
    }

    //       [D]    
    //   [N] [C]    
    //   [Z] [M] [P]
    //    1   2   3 
    fn parse_ship(ship_graphic: &str) -> Ship {

        // determine the number of stacks. note all lines are space-padded to the end
        let levels = ship_graphic.split('\n').collect::<Vec<_>>();
        let stack_count = (levels[0].len() + 1) / 4;

        // allocate space, adding 1 so we can use 1-based indices throughout the code
        let mut ship : Vec<Vec<Crate>> = vec![vec![]; stack_count + 1];

        // work bottom up, skipping 1 because the last line holds the stack numbers
        for level in levels.iter().rev().skip(1) {

            // rust has real Unicode strings so we can't easily get a letter at a certain
            // index, but the input is normal ASCII so we can treat it as a slice of bytes
            let chars = level.as_bytes();

            // go through this level of crates, skipping blank chars (no crate)
            for i in 1..=stack_count {

                // pick out the crate letter in between the square brackets
                let letter = chars[(i-1)*4 + 1];
                if letter == b' ' { continue }

                ship[i].push(letter as char);
            }
        }

        ship
    }

    // "move 1 from 2 to 3"
    // "move 4 from 5 to 6"
    fn parse_moves(moves: &str) -> Vec<Move> {
        moves.lines()
             .map(Move::from)
             .collect()
    }
}

impl Move {    
    fn from(line: &str) -> Self {
        let split = line.split_whitespace().collect::<Vec<_>>();

        // "move 1 from 2 to 3"
        Move {
            amount: split[1].parse().unwrap(),
            from  : split[3].parse().unwrap(),
            to    : split[5].parse().unwrap()
        }
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ship() {
        let example =  "    [D]    \n\
                        [N] [C]    \n\
                        [Z] [M] [P]\n\
                         1   2   3 ";

        let ship = Input::parse_ship(&example);

        assert_eq!(ship.len(), 1+3);
        assert_eq!(ship[1], vec!['Z','N']);
        assert_eq!(ship[2], vec!['M','C','D']);
        assert_eq!(ship[3], vec!['P']);
    }

    #[test]
    fn test_parse_move() {
        let m = Move::from("move 1 from 2 to 3");
        assert_eq!(m.amount, 1);
        assert_eq!(m.from, 2);
        assert_eq!(m.to, 3);
    }
}

/*  $ cargo run
    Compiling day_05 v0.1.0 (C:\Users\Jason\Documents\GitHub\aoc-2022\day_05)
        Finished dev [unoptimized + debuginfo] target(s) in 0.44s
        Running `target\debug\day_05.exe`
    Part 1: SHMSDGZVC
    Part 2: VRZGHDFBQ
*/
