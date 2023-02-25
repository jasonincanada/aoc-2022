/*  https://adventofcode.com/2022/day/17  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { jet_pattern: String }

struct Pos {
    row: usize,
    col: usize
}

struct Move {
    row: i32,
    col: i32
}

type Shape = Vec<Pos>;

struct Chamber {
    grid: Vec<Vec<bool>>
}


// simulate falling rocks until we have 2022 settled rocks
fn part1(input: &Input) -> usize {
    let mut chamber = Chamber::new();
    let shapes = shapes();
    
    // count how many rocks come to a stop
    let mut stopped = 0;

    // our repeating iterators
    let mut rocks = repeat(shapes.iter());
    let mut jet   = repeat(input.jet_pattern.chars());
    
    while stopped <= 2022 {
        let rock = rocks.next().unwrap();

        // start a new rock 3 rows above the pile
        let mut location = Pos {
            row: chamber.top_row() + 3,   // for some reason this gives the wrong answer if
            col: 2                        // i use top_occupied_row() instead of top_row(),
        };                                // there shouldn't be a difference

        loop {
            let jet = jet.next().unwrap();

            chamber.move_by_jet(rock, &mut location, jet);

            if chamber.move_down(rock, &mut location) {
                continue
            }

            // it couldn't move down, so lock it in place
            chamber.situate(rock, &location);
            break;
        }

        stopped += 1;

        // println!("Stopped {}, Rows {}", stopped, chamber.top_occupied_row());
    }

    chamber.top_occupied_row()
}

// iterate a trillion times
fn part2(_input: &Input) -> usize {
    let _times: i64 = 1_000_000_000_000;

    0
}


/* Chamber */

const WIDTH: usize = 7;

impl Chamber {
    fn move_by_jet(&self, rock    : &Shape,
                          location: &mut Pos,
                          jet     : char) -> bool
    {
        let relative = match jet {
            '>' => Move { row: 0, col:  1 },
            '<' => Move { row: 0, col: -1 },
             _  => panic!("unknown jet direction")
        };

        self.move_if_possible(rock, location, relative)
    }

    fn move_down(&self, rock    : &Shape,
                        location: &mut Pos) -> bool
    {
        self.move_if_possible(rock,
                              location,
                              Move { row: -1, col: 0 })
    }

    // move if there's room to and return true if we did
    fn move_if_possible(&self, rock    : &Shape,
                               location: &mut Pos,
                               relative: Move) -> bool
    {
        if self.can_move(rock, location, &relative) {
            location.row = (location.row as i32 + relative.row) as usize;
            location.col = (location.col as i32 + relative.col) as usize;
            return true
        }

        false
    }

    fn can_move(&self, rock    : &Shape,
                       location: &Pos,
                       relative: &Move) -> bool
    {
        rock.iter()
            .all(|&Pos {row, col}| self.is_space(location.row as i32 + row as i32 + relative.row,
                                                 location.col as i32 + col as i32 + relative.col))
    }

    fn is_space(&self, row: i32, col: i32) -> bool {
        if row < 0 { return false }
        if col < 0 { return false }
        if col >= WIDTH as i32 { return false }

        if row as usize >= self.grid.len() { return true }

        ! self.grid[row as usize][col as usize]
    }

    // lock a rock into place by filling its grid coordinates with true
    fn situate(&mut self, rock    : &Shape,
                          location: &Pos)
    {
        for pos in rock {
            let row = pos.row + location.row;
            let col = pos.col + location.col;

            // allocate space above as necessary
            while row >= self.grid.len() {
                self.grid.push(vec![false; WIDTH as usize]);
            }

            self.grid[row][col] = true;
        }
    }

    // return the highest row index of a stopped rock
    fn top_occupied_row(&self) -> usize {
        if self.grid.is_empty() { return 0 }

        for row in (0..self.grid.len()-1).rev() {
            if self.grid[row].iter()
                             .any(|cell| *cell)
            {
                return row
            }
        }

        0
    }
    
    fn top_row(&self) -> usize {
        self.grid.len()
    }

    fn new() -> Self {
        Chamber { grid: vec![] }
    }
}

fn shapes() -> Vec<Shape> {

    // ####
    let line_across = vec![
        Pos { row: 0, col: 0},
        Pos { row: 0, col: 1},
        Pos { row: 0, col: 2},
        Pos { row: 0, col: 3},
    ];

    // .#.
    // ###
    // .#.
    let cross = vec![
        Pos { row: 2, col: 1},
        Pos { row: 1, col: 0},
        Pos { row: 1, col: 1},
        Pos { row: 1, col: 2},
        Pos { row: 0, col: 1},
    ];

    // ..#
    // ..#
    // ###
    let l = vec![
        Pos { row: 2, col: 2},
        Pos { row: 1, col: 2},
        Pos { row: 0, col: 0},
        Pos { row: 0, col: 1},
        Pos { row: 0, col: 2},
    ];

    // #
    // #
    // #
    // #
    let line_down = vec![
        Pos { row: 3, col: 0},
        Pos { row: 2, col: 0},
        Pos { row: 1, col: 0},
        Pos { row: 0, col: 0},
    ];

    // ##
    // ##
    let square = vec![
        Pos { row: 1, col: 0},
        Pos { row: 1, col: 1},
        Pos { row: 0, col: 0},
        Pos { row: 0, col: 1},
    ];

    vec![line_across, cross, l, line_down, square]
}


/* Repeater Iterator  */

struct Repeater<I: Iterator> {
    iter: I,
    holding: Vec<I::Item>,
    idx: usize
}

impl<I> Iterator for Repeater<I>
where
    I: Iterator,
    I::Item: Copy  // the item type doesn't matter as long as we can make copies of it
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {

        match self.iter.next() {

            // make copies of the items before returning them so when the underlying
            // iterator runs out we can start returning clones of them
            Some(item) => {
                self.holding.push(item.clone());
                Some(item)
            },

            // underlying iterator is exhausted, so start iterating over
            // the copies we made
            None => {
                let next = self.holding[self.idx].clone();

                self.idx += 1;
                if self.idx >= self.holding.len() {
                    self.idx = 0;
                }

                Some(next)
            }
        }
    }
}

// take an iterator and return a new one that repeats that iterator's items forever
fn repeat<I: Iterator>(iter: I) -> Repeater<I> {
    Repeater {
        iter,
        holding: vec![],
        idx: 0
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
            jet_pattern: s.to_string()
        }
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeater_iterator() {
        let vec: Vec<i32> = vec![1,2,3];

        let mut repeater = repeat(vec.iter());

        assert_eq!(repeater.next(), Some(&1));
        assert_eq!(repeater.next(), Some(&2));
        assert_eq!(repeater.next(), Some(&3));
        assert_eq!(repeater.next(), Some(&1));
        assert_eq!(repeater.next(), Some(&2));
        assert_eq!(repeater.next(), Some(&3));
        assert_eq!(repeater.next(), Some(&1));
        assert_eq!(repeater.next(), Some(&2));
    }

    #[test]
    fn test_can_move() {
        let rock = &shapes()[0];
        let location = Pos { row: 41, col: 0 };
        let relative = Move { row: 0, col: -1 };

        let chamber = Chamber::new();

        assert!(! chamber.can_move(rock, &location, &relative))
    }

    #[test]
    fn test_part1() {
       assert_eq!(part1(&get_example()), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_example()), 1514285714288);
    }

    fn get_example() -> Input {
        Input::from_string(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")
    }
}
