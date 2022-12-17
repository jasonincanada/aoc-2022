/*  https://adventofcode.com/2022/day/14  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { paths : Vec<Path>  }
struct Path  { points: Vec<Point> }

struct Point {
    col: usize,
    row: usize
}

// count how many grains of sand come to rest before they start sliding off forever
fn part1(input: &Input) -> usize {
    falling_sand(input, 1)
}

// count how many come to rest when we include the floor we added beneath the rock paths
fn part2(input: &Input) -> usize {
    falling_sand(input, 2)
}

fn falling_sand(input: &Input, part: usize) -> usize {
    let mut cave = build_cave(&input.paths);

    let lowest = input.paths.iter()
                            .flat_map(|path| path.points.iter().map(|Point {col:_, row}| *row))
                            .max().unwrap();

    let mut sand = Point {col: 500, row: 0};
    let mut rest = 0;

    loop {
        if part == 1 && sand.row >= lowest { break }

        // if there's air directly below
        if cave[sand.row+1][sand.col] == Type::Air {
            sand.row += 1;
            continue
        }

        // try below and to the left
        if cave[sand.row+1][sand.col-1] == Type::Air {
            sand.row += 1;
            sand.col -= 1;
            continue
        }

        // below and to the right
        if cave[sand.row+1][sand.col+1] == Type::Air {
            sand.row += 1;
            sand.col += 1;
            continue
        }

        // nowhere for this sand to fall so it settles here
        cave[sand.row][sand.col] = Type::Sand;
        rest += 1;

        // part 2 ends when the grain of sand couldn't fall at all
        if part == 2 && sand.row == 0 && sand.col == 500 { break }

        // start a new grain of sand
        sand.row = 0;
        sand.col = 500;
    }

    rest
}

#[derive(Clone, Eq, PartialEq)]
enum Type {
    Air,
    Rock,
    Sand
}

type Cave = Vec<Vec<Type>>;

fn build_cave(paths: &[Path]) -> Cave {
    let rightest = paths.iter()
                        .flat_map(|path| path.points.iter().map(|Point {col, row:_}| col))
                        .max().unwrap();

    let lowest   = paths.iter()
                        .flat_map(|path| path.points.iter().map(|Point {col:_, row}| row))
                        .max().unwrap();

    // for part 2 we need an "infinite" floor along the bottom, or for our purposes
    // enough floor to support a triangle of falling sand
    let rightest = rightest + lowest;

    let mut cave: Cave = vec![ vec![Type::Air; rightest+2]; lowest+3 ];

    // fill in the rocks along the paths
    for path in paths {
        for pair in path.points.windows(2) {
            let (left, right) = min_max(pair[0].col, pair[1].col);
            let (bottom, top) = min_max(pair[0].row, pair[1].row);

            for col in left..=right {
            for row in bottom..=top {
                cave[row][col] = Type::Rock;
            }}
        }
    }

    // add the floor for part 2
    for col in 0..=rightest+1 {
        cave[lowest+2][col] = Type::Rock;
    }

    cave
}

fn min_max(a: usize, b: usize) -> (usize, usize) {
    (a.min(b), a.max(b))
}


/* Parsing  */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            paths: s.lines()
                    .map(Path::from_string)
                    .collect()
        }
    }
}

impl Path {
    // 498,4 -> 498,6 -> 496,6
    fn from_string(s: &str) -> Self {
        Path {
            points: s.split(" -> ")
                     .map(Point::from_string)
                     .collect()
        }

    }
}

impl Point {
    // 498,4
    fn from_string(s: &str) -> Self {
        let (col, row) = s.split_once(',').unwrap();

        Point {
            row: row.parse().unwrap(),
            col: col.parse().unwrap()
        }
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_example()), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_example()), 93);
    }

    fn get_example() -> Input {
        Input::from_string(
            "498,4 -> 498,6 -> 496,6\n\
             503,4 -> 502,4 -> 502,9 -> 494,9"
        )
    }
}
