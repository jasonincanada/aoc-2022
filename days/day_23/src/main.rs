/*  https://adventofcode.com/2022/day/23  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { grove : Grid }
struct Grove { grid  : Grid }

type Grid = Vec<Vec<char>>;

// simulate 10 rounds, then count the number of empty tiles between elves
fn part1(input: &Input) -> usize {
    let grove = Grove::from(input.grove.clone());
    simulate(grove, 1)
}

// keep simulating until no elf gets to move during a round
fn part2(input: &Input) -> usize {
    let grove = Grove::from(input.grove.clone());
    simulate(grove, 2)
}


/* Simulate */

fn simulate(mut grove: Grove, part: usize) -> usize {
    let mut proposals = get_proposals();

    for round in 1.. {
        grove.wrap_with_ground_tiles();

        // first half of round, collect the coordinates the elves propose to move to
        let proposed: HashMap<Pos, Vec<Elf>> = grove.get_proposed_moves(&proposals);

        // second half, an elf moves if they're the only one to propose moving to that tile
        let mut elves_moved = false;

        for (move_to, elves) in proposed.iter() {
            // no elf moves to this position if more than one proposed to
            if elves.len() > 1 { continue }

            grove.move_elf(&elves[0], move_to);
            elves_moved = true;
        }

        if part == 1 && round == 10  { break }
        if part == 2 && !elves_moved { return round }

        // move the first proposal in the list to the end
        cycle(&mut proposals);
    }

    grove.count_empty_tiles()
}

// put the first item of a vector at the end
fn cycle<T>(vec: &mut Vec<T>) {
    assert!(!vec.is_empty());

    let first = vec.remove(0);
    vec.push(first);
}


/* Grove */

impl Grove {
    // wrap a rectangle of empty tiles around the grid. called at the start of
    // every round to make sure there's enough space to index within bounds
    fn wrap_with_ground_tiles(&mut self) {
        for row in self.grid.iter_mut() {
            row.insert(0, GROUND);
            row.push(GROUND);
        }

        self.grid.insert(0, vec![ GROUND; self.grid[0].len()]);
        self.grid.push(     vec![ GROUND; self.grid[0].len()]);
    }

    // first half of the round gets the list of positions the elves want to move to
    fn get_proposed_moves(&self, proposals: &[Proposal]) -> HashMap<Pos, Vec<Elf>> {

        // first build a vector of elves and their proposed moves
        let mut moves: Vec<(Elf, Pos)> = vec![];

        for (r, row) in self.grid.iter().enumerate() {
        for (c, &tile) in row.iter().enumerate() {
            if tile == GROUND { continue }

            let pos = Pos { row: r, col: c };
            if !self.elves_around(&pos) { continue }

            for Proposal { moving, criteria } in proposals.iter() {
                if !self.elves_at(&pos, criteria) {
                    moves.push(
                        (pos.clone(),
                         pos.move_by(moving))
                    );
                    break
                }
            }
        }}

        // collect the moves into a hashmap to tally the number of proposals per position
        let mut hashmap: HashMap<Pos, Vec<Elf>> = HashMap::new();

        for m in moves {
            let elves = hashmap.entry(m.1)
                               .or_insert_with(Vec::new);
            elves.push(m.0);
        }

        hashmap
    }

    // check if there are any elves immediately surrounding the elf at this pos
    fn elves_around(&self, pos: &Pos) -> bool {
           self.grid[pos.row-1][pos.col-1] == ELF
        || self.grid[pos.row-1][pos.col  ] == ELF
        || self.grid[pos.row-1][pos.col+1] == ELF
        || self.grid[pos.row  ][pos.col-1] == ELF
        || self.grid[pos.row  ][pos.col+1] == ELF
        || self.grid[pos.row+1][pos.col-1] == ELF
        || self.grid[pos.row+1][pos.col  ] == ELF
        || self.grid[pos.row+1][pos.col+1] == ELF
    }

    // check if there are any elves in the given relative positions
    fn elves_at(&self, pos: &Pos, moves: &[Move]) -> bool {
        moves.iter()
             .any(|m| { let p = pos.move_by(m);
                        self.grid[p.row][p.col] == ELF
              })
    }

    // move an elf by mutably swapping a '#' and '.' in the grid
    fn move_elf(&mut self, elf: &Pos, to: &Pos) {
        assert!(self.grid[to .row][to .col] == GROUND);
        assert!(self.grid[elf.row][elf.col] == ELF);
        self.grid[to .row][to .col] = ELF;
        self.grid[elf.row][elf.col] = GROUND;
    }

    // bound the elves in the smallest rectangle that covers them all and count the empty tiles
    fn count_empty_tiles(&self) -> usize {
        let (left, right, top, bottom) = self.rectangle_around();

        let mut empty_tiles = 0;
    
        for row in top..=bottom {
        for col in left..=right {
            if self.grid[row][col] == GROUND {
                empty_tiles += 1;
            }
        }
        }

        empty_tiles
    }

    // get the smallest rectangle around all the elves
    fn rectangle_around(&self) -> (usize, usize, usize, usize) {
        let elves: Vec<Pos> = self.grid.iter()
                                       .enumerate()
                                       .flat_map(|(r, row)| row.iter()
                                                               .enumerate()
                                                               .filter(|(_, &tile)| tile == ELF)
                                                               .map(|(c, _)| {
                                                                   Pos { row: r, col: c}
                                                               })
                                                               .collect::<Vec<Pos>>())
                                       .collect();

        assert!( !elves.is_empty() );

        let cols: Vec<usize> = elves.iter().map(|p| p.col).collect();
        let rows: Vec<usize> = elves.iter().map(|p| p.row).collect();

        let left   = *cols.iter().min().unwrap();
        let right  = *cols.iter().max().unwrap();
        let top    = *rows.iter().min().unwrap();
        let bottom = *rows.iter().max().unwrap();

        (left, right, top, bottom)
    }
}


/* Static Data */

// the four proposals listed in the problem description
fn get_proposals() -> Vec<Proposal> {
    vec![
        // north
        Proposal {
            moving  : Move::new(-1, 0),
            criteria: vec![ Move::new(-1, -1),
                            Move::new(-1, 0),
                            Move::new(-1, 1) ]
        },
        // south
        Proposal {
            moving  : Move::new(1, 0),
            criteria: vec![ Move::new(1, -1),
                            Move::new(1, 0),
                            Move::new(1, 1) ]
        },
        // west
        Proposal {
            moving  : Move::new(0, -1),
            criteria: vec![ Move::new(-1, -1),
                            Move::new( 0, -1),
                            Move::new( 1, -1) ]
        },
        // east
        Proposal {
            moving  : Move::new(0, 1),
            criteria: vec![ Move::new(-1, 1),
                            Move::new( 0, 1),
                            Move::new( 1, 1) ]
        }
    ]
}


/* Types */

type Elf  = Pos;

const ELF    : char = '#';
const GROUND : char = '.';

#[derive(Clone, Eq, Hash, PartialEq)]
struct Pos {
    row: usize,
    col: usize
}

struct Move {
    rows: i32,
    cols: i32
}

struct Proposal {
    moving  : Move,
    criteria: Vec<Move>
}


/* Constructors */

impl Grove {
    fn from(grid: Grid) -> Self {
        Grove { grid }
    }
}

impl Pos {
    fn move_by(&self, move_by: &Move) -> Pos {
        assert!(self.row as i32 + move_by.rows >= 0);
        assert!(self.col as i32 + move_by.cols >= 0);

        Pos {
            row: (self.row as i32 + move_by.rows) as usize,
            col: (self.col as i32 + move_by.cols) as usize
        }
    }
}

impl Move {
    fn new(rows: i32, cols: i32) -> Self {
        Move {
            rows,
            cols
        }
    }
}


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            grove: s.lines()
                    .map(|line| line.chars().collect())
                    .collect()
        }
    }
}


/* Imports */

use std::collections::HashMap;


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_part1() { assert_eq!(part1(&get_example()), 110); }
    #[test] fn test_part2() { assert_eq!(part2(&get_example()), 20); }

    fn get_example() -> Input {
        Input::from_string(
            "....#..\n\
             ..###.#\n\
             #...#.#\n\
             .#...##\n\
             #.###..\n\
             ##.#.##\n\
             .#..#.."
        )
    }
}

/*  $ time target/release/day_23.exe
    Part 1: 4123
    Part 2: 1029

    real    0m1.309s
*/
