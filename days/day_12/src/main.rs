/*  https://adventofcode.com/2022/day/12  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input {
    heightmap: HeightMap,
    start    : Pos,
    end      : Pos
}

type HeightMap = Vec<Vec<char>>;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Pos {
    row: usize,
    col: usize
}

// find the shortest path from start to end (backwards using the opposite stepping logic)
fn part1(input: &Input) -> u32 {
    let distances = dijkstra(&input.heightmap, &input.end);
    distances[input.start.row][input.start.col]
}

// find the shortest path to any 'a' starting at the end again
fn part2(input: &Input) -> u32 {
    let distances = dijkstra(&input.heightmap, &input.end);

    positions_of('a', &input.heightmap)
        .into_iter()
        .map(|pos| distances[pos.row][pos.col])
        .min().unwrap()
}

// Dijkstra's shortest path algorithm from the pseudocode on Wikipedia:
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
//
// start at the given position and find the shortest path to all reachable positions
fn dijkstra(heightmap: &HeightMap,
            start: &Pos) -> Vec<Vec<u32>>
{
    let rows = heightmap.len();
    let cols = heightmap[0].len();

    let mut distances: Vec<Vec<u32>>         = vec![ vec![u32::MAX-1; cols]; rows ];  // u32 distances
    let mut previous : Vec<Vec<Option<Pos>>> = vec![ vec![None      ; cols]; rows ];  // Pos coordinates

    // queue up every coordinate
    use std::collections::HashSet;
    let mut queue: HashSet<Pos> = HashSet::new();

    for row in 0..rows {
    for col in 0..cols {
        queue.insert(Pos {row, col});
    }}

    // set the first known distance: 0 from the start to the start
    distances[start.row][start.col] = 0;

    while !queue.is_empty() {

        // find the position in the queue with shortest distance from start
        let u = queue.iter()
                     .min_by(|a, b| distances[a.row][a.col].cmp(&distances[b.row][b.col]))
                     .unwrap()
                     .clone();

        queue.remove(&u);

        let neighbours: Vec<Pos> =
            get_neighbours(&u, rows, cols)
                .into_iter()
                .filter(|pos| queue.contains(pos))
                .filter(|pos| can_step_to(pos, &u, heightmap))
                .collect();

        for v in neighbours {
            // a step to a neighbouring square is always a distance of 1
            let alt = distances[u.row][u.col] + 1;

            if alt < distances[v.row][v.col] {
                distances[v.row][v.col] = alt;
                 previous[v.row][v.col] = Some(u.clone());
            }
        }
    }

    distances
}

// can we make a step on our grid. since we do our searches backwards, the logic is opposite
// to the problem description, ie, can we make this step, is it no more than 1 lower
fn can_step_to(to:   &Pos,
               from: &Pos,
               heightmap: &HeightMap) -> bool
{
      heightmap[from.row][from.col] as i32
    - heightmap[to.row][to.col] as i32
   <= 1
}

// get the neighbouring positions of the passed position, excluding any off the map
fn get_neighbours(p: &Pos, rows: usize, cols: usize) -> Vec<Pos> {
    vec![
        (p.row as i32 - 1, p.col as i32    ),  // all this casting because usizes can't
        (p.row as i32 + 1, p.col as i32    ),  // even temporarily be negative
        (p.row as i32    , p.col as i32 - 1),
        (p.row as i32    , p.col as i32 + 1),
    ].into_iter()
     .filter(|(row, col)|    row >= &0 && row < &(rows as i32)
                          && col >= &0 && col < &(cols as i32))
     .map(|(row, col)| Pos { row: row as usize,
                             col: col as usize})
     .collect()
}

// get all positions of a certain elevation in a heightmap
fn positions_of(elevation: char, heightmap: &HeightMap) -> Vec<Pos> {
    let mut positions: Vec<Pos> = vec![];

    for row in 0..heightmap.len() {
    for col in 0..heightmap[0].len() {
        if heightmap[row][col] == elevation {
            positions.push(Pos {row, col});
        }
    }}

    positions
}


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        let mut heightmap: HeightMap = vec![];
        let mut start = Pos {row: 0, col: 0};
        let mut end   = Pos {row: 0, col: 0};

        for (row, line) in s.split('\n').enumerate() {
            let mut heights = vec![];

            for (col, char) in line.chars().enumerate() {
                match char {
                    'S' => {
                        heights.push('a');
                        start.row = row;
                        start.col = col;
                    },
                    'E' => {
                        heights.push('z');
                        end.row = row;
                        end.col = col;
                    },
                    elevation => {
                        heights.push(elevation);
                    }
                }
            }

            heightmap.push(heights);
        }

        Input {
            heightmap,
            start,
            end
        }
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_example()), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_example()), 29);
    }

    #[test]
    fn test_parse() {
        let map = get_example();
        assert_eq!(map.start.row, 0);
        assert_eq!(map.start.col, 0);
        assert_eq!(map.end.row, 2);
        assert_eq!(map.end.col, 5);
        assert_eq!(map.heightmap[0][0], 'a');
        assert_eq!(map.heightmap[2][5], 'z');
        assert_eq!(map.heightmap[1][1], 'b');
    }

    fn get_example() -> Input {
        Input::from_string(
            "Sabqponm\n\
             abcryxxl\n\
             accszExk\n\
             acctuvwj\n\
             abdefghi"
        )
    }
}

/*  $ time target/release/day_12.exe
    Part 1: 534
    Part 2: 525

    real    0m0.170s
*/
