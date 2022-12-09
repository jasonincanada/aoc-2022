/*  https://adventofcode.com/2022/day/9  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { moves: Vec<Move> }

struct Move {
    direction: char,
    steps: usize
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coord {
    row: i32,
    col: i32
}

fn part1(input: &Input) -> usize { pull_rope_length(&input.moves,  2) }
fn part2(input: &Input) -> usize { pull_rope_length(&input.moves, 10) }

// count the number of coordinates the tail of the rope visits as it's pulled around a grid
fn pull_rope_length(moves: &[Move], length: usize) -> usize {

    // start the whole rope bunched up on the 0,0 coordinate
    let mut rope: Vec<Coord> = vec![Coord {row:0,col:0}; length];

    // save the location of the tail after every move
    let mut visited = std::collections::HashSet::new();
    visited.insert(Coord {row:0,col:0});

    for Move {direction, steps} in moves.iter() {
        for _ in 1..=*steps {

            // move the first knot by one step and catch the rest up
            match direction {
                'R' => { rope[0].col += 1; deslackify(&mut rope); },
                'L' => { rope[0].col -= 1; deslackify(&mut rope); },
                'U' => { rope[0].row += 1; deslackify(&mut rope); },
                'D' => { rope[0].row -= 1; deslackify(&mut rope); },
                 _  =>   panic!("Unexpected direction")
            }

            // remember the location of the tail after this step
            visited.insert(rope[length-1].clone());
        }
    }

    visited.len()
}

// move knots of the rope as needed to remove slack
fn deslackify(rope: &mut Vec<Coord>) {

    for i in 0..rope.len()-1 {
        let leader   = rope[i  ].clone();
        let follower = rope[i+1].clone();

        if leader.col - follower.col > 1 {
                                           rope[i+1].col += 1;
            if leader.row > follower.row { rope[i+1].row += 1; }
            if leader.row < follower.row { rope[i+1].row -= 1; }
        }
        else if follower.col - leader.col > 1 {
                                           rope[i+1].col -= 1;
            if leader.row > follower.row { rope[i+1].row += 1; }
            if leader.row < follower.row { rope[i+1].row -= 1; }
        }
        else if leader.row - follower.row > 1 {
                                           rope[i+1].row += 1;
            if leader.col > follower.col { rope[i+1].col += 1; }
            if leader.col < follower.col { rope[i+1].col -= 1; }
        }
        else if follower.row - leader.row > 1 {
                                           rope[i+1].row -= 1;
            if leader.col > follower.col { rope[i+1].col += 1; }
            if leader.col < follower.col { rope[i+1].col -= 1; }
        }
    }
}

/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(&contents)
    }

    fn from_string(s: &str) -> Input {
        Input {
            moves: s.trim()
                    .lines()
                    .map(Move::from_string)
                    .collect()
        }
    }
}

impl Move {
    fn from_string(line: &str) -> Self {
        let (dir, steps) = line.split_once(' ').unwrap();

        Move {
            direction: dir.chars().next().unwrap(),
            steps: steps.parse().unwrap()
        }
    }
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = get_example1();
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_example1()), 1);
        assert_eq!(part2(&get_example2()), 36);
    }

    fn get_example1() -> Input {
        Input::from_string(
            "R 4\n\
             U 4\n\
             L 3\n\
             D 1\n\
             R 4\n\
             D 1\n\
             L 5\n\
             R 2"
        )
    }

    fn get_example2() -> Input {
        Input::from_string(
            "R 5\n\
             U 8\n\
             L 8\n\
             D 3\n\
             R 17\n\
             D 10\n\
             L 25\n\
             U 20"
        )
    }
}
