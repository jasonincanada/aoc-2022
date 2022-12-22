/*  https://adventofcode.com/2022/day/22  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input {
    map  : Map,
    path : Vec<Move>
}

type Map = Vec<Vec<char>>;

#[derive(Debug, PartialEq)]
enum Move {
    Forward(usize),
    TurnRight,
    TurnLeft
}

enum Facing {
    Right,
    Down,
    Left,
    Up
}

struct Pos {
    row: usize,
    col: usize
}

impl Pos {
    fn to_right(&self) -> Pos { Pos { row: self.row    , col: self.col + 1 }}
    fn to_left(&self)  -> Pos { Pos { row: self.row    , col: self.col - 1 }}
    fn to_down(&self)  -> Pos { Pos { row: self.row + 1, col: self.col     }}
    fn to_up(&self)    -> Pos { Pos { row: self.row - 1, col: self.col     }}
}

// navigate according to the plan, wrapping around in the obvious way
fn part1(input: &Input) -> usize {
    let mut pos    = Pos { row: 1, col: 1 };
    let mut facing = Facing::Right;

    // start at the left-most open tile in the top row
    pos.col = input.map[1].iter()
                          .position(|&c| c == '.')
                          .unwrap();

    for m in input.path.iter() {
        match m {
            Move::Forward(mut steps) => {
                
                while steps > 0
                {
                    if let Some(new_pos) = step_to(&input.map, &facing, &pos) {
                        pos = new_pos;
                    } else {
                        break
                    }

                    steps -= 1;
                }
            },
            dir => facing = turn(&facing, dir)
        }
    }

      pos.row * 1000
    + pos.col * 4
    + value_of(&facing)
}

// if we can make this step on the map, return the new coordinates, otherwise None
fn step_to(map   : &Map,
           facing: &Facing,
           pos   : &Pos) -> Option<Pos>
{
    let mut new = match facing {
        Facing::Right => pos.to_right(),
        Facing::Down  => pos.to_down(),
        Facing::Left  => pos.to_left(),
        Facing::Up    => pos.to_up()
    };

    // if this took us off the edge, wrap around
    if map[new.row][new.col] == ' ' {
        
        match facing
        {
            Facing::Right => new.col = map[new.row].iter(). position(|c| !c.is_whitespace()).unwrap(),
            Facing::Left  => new.col = map[new.row].iter().rposition(|c| !c.is_whitespace()).unwrap(),

            Facing::Down  => new.row = map.iter()
                                          .map(|row| row[new.col])
                                          .position(|c| !c.is_whitespace())
                                          .unwrap(),

            Facing::Up    => new.row = map.iter()
                                          .map(|row| row[new.col])
                                          .rposition(|c| !c.is_whitespace())
                                          .unwrap()
        };
    }

    // if we'd run into a wall, we can't make the step
    if map[new.row][new.col] == '#' { return None }

    Some(new)
}

fn turn(facing: &Facing, turn: &Move) -> Facing {
    use Facing::*;
    use Move::*;

    match (facing, turn) {
        (Right, TurnRight) => Down,
        (Right, TurnLeft)  => Up,
        (Down, TurnRight)  => Left,
        (Down, TurnLeft)   => Right,
        (Left, TurnRight)  => Up,
        (Left, TurnLeft)   => Down,
        (Up, TurnRight)    => Right,
        (Up, TurnLeft)     => Left,
        (_, Forward(_))    => panic!("can't turn forward")
    }
}

fn value_of(facing: &Facing) -> usize {
    match facing {
        Facing::Right => 0,
        Facing::Down  => 1,
        Facing::Left  => 2,
        Facing::Up    => 3
    }
}

// use the same plan but consider the map an unfolded cube, wrap around accordingly
fn part2(_input: &Input) -> u32 {
    0
}


/* Imports */

use regex::Regex;


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim_end())
    }

    fn from_string(s: &str) -> Self {
        let (map, path) = s.split_once("\n\n").unwrap();

        Input {
             map: Input::map_from(map),
            path: Input::path_from(path)
        }
    }

    fn map_from(s: &str) -> Vec<Vec<char>> {
        let mut map: Vec<Vec<char>> =
            s.lines()
             .map(|line| line.chars().collect())
             .collect();

        // make sure there's spaces all around the exterior and the rows are all
        // padded out to the right to make all rows the same width
        let width = map.iter()
                       .map(|line| line.len())
                       .max().unwrap();

        // first and last row of all spaces
        map.insert(0, vec![' '; width + 2]);
        map.push(vec![' '; width + 2]);

        for row in map.iter_mut() {
            row.insert(0, ' ');

            while row.len() < width + 2 {
                row.push(' ');
            }
        }

        map
    }

    // "10R5L5"
    fn path_from(s: &str) -> Vec<Move> {
        let regex = Regex::new(r"(L|R|\d+)").unwrap();
        let mut moves: Vec<Move> = vec![];

        for cap in regex.captures_iter(s) {
            moves.push(
                match &cap[0] {
                    "L" => Move::TurnLeft,
                    "R" => Move::TurnRight,
                    num => Move::Forward(num.parse().unwrap())
                });
        }

        moves
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_path() {
        let expected = vec![
            Move::Forward(10),
            Move::TurnRight,
            Move::Forward(5),
            Move::TurnLeft,
            Move::Forward(6),
        ];
        assert_eq!(Input::path_from("10R5L6"), expected);
    }

    #[test] fn test_part1() { assert_eq!(part1(&get_example()), 6032); }
    #[test] fn test_part2() { assert_eq!(part2(&get_example()), 5031); }

    fn get_example() -> Input {
        Input::from("example.txt")
    }
}
