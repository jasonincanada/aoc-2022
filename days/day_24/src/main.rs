/*  https://adventofcode.com/2022/day/24  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { valley: ValleyMap<bool> }

// compute the fastest way through the valley without getting caught in a blizzard
fn part1(input: &Input) -> usize {

    // start out at the starting tile at the starting time
    let start = Tile::Start(0);

    // set up our function for getting a vertex's neighbours
    let follow_edges = |tile: &Tile| get_neighbours(&input.valley, tile);

    // breadth-first search to goal positions
    let distances: ValleyMap<Option<usize>> = bfs(&input.valley, follow_edges, start);

    // get the fastest time to the end goal
    (0..input.valley.weather_maps)
        .into_iter()
        .map(|time| distances[&Tile::Goal(time)].unwrap())
        .min()
        .unwrap_or(0)
}


/* ValleyMap */

// special struct that accounts for each possible place in a valley at any given time,
// including the start and end positions
struct ValleyMap<T> {
    tiles: Vec<T>,

    // size of the interior of the valley (blizzard area)
    height: usize,
    width:  usize,

    // number of distinct blizzard maps. they repeat after width*height,
    // even sooner if gcd(width, height) > 1
    weather_maps: usize
}

// a tile is an index into our 3D grid (two space and one time). Start/Goal represent the
// fixed start and goal positions, where Valley is somewhere on the main grid at some time
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Valley(usize, usize, usize), // time, row, col
    Start(usize),                // time
    Goal(usize)                  // time
}

// ValleyMap uses a custom indexing system because the grid isn't quite rectangle due to the
// start and end positions jutting out at the top/bottom, and because time is a dimension,
// and we want to store it all in one linear vector for constant-time access to elements.
// it's generic over a type parameter T because we use it for two different things:
// tracking which positions at which times are not covered in a blizzard (bool), and then
// in the shortest path algorithm, it tracks minimum distance to each position (usize)
impl<T> Index<&Tile> for ValleyMap<T> {
    type Output = T;

    fn index(&self, index: &Tile) -> &Self::Output {
        let generation_size =   self.width
                              * self.height
                              + 2; // start/goal positions

        match index {
            Tile::Valley(time, row, col)
                              => &self.tiles[ (time % self.weather_maps) * generation_size
                                              + row * self.width
                                              + col ],

            // tuck the start and end cells right after the valley
            Tile::Start(time) => &self.tiles [ (time % self.weather_maps + 1)
                                               * generation_size - 2 ],

            Tile::Goal(time)  => &self.tiles [ (time % self.weather_maps + 1)
                                               * generation_size - 1 ],
        }
    }
}

// same as above but return a &mut ref to the tile so we can update it
impl<T> IndexMut<&Tile> for ValleyMap<T> {
    fn index_mut(&mut self, index: &Tile) -> &mut Self::Output {
        let generation_size = self.width * self.height + 2;

        match index {
            Tile::Valley(time, row, col)
                              => &mut self.tiles[ (time % self.weather_maps) * generation_size
                                                  + row * self.width
                                                  + col ],

            Tile::Start(time) => &mut self.tiles [ (time % self.weather_maps + 1)
                                                   * generation_size - 2 ],

            Tile::Goal(time)  => &mut self.tiles [ (time % self.weather_maps + 1)
                                                   * generation_size - 1 ],
        }
    }
}

impl<T> ValleyMap<T> {
    // copy the shape of the valley and set all tiles to usize::MAX-1
    fn to_initialized_distances(&self) -> ValleyMap<Option<usize>> {
        ValleyMap {
            tiles : self.tiles.iter()
                        .map(|_| None )
                        .collect(),
            width : self.width,
            height: self.height,
            weather_maps: self.weather_maps
        }
    }
}

// get the available (no blizzard) neighbours in a ValleyMap<bool> in the next unit of time
fn get_neighbours(valley: &ValleyMap<bool>,
                  tile  : &Tile) -> Vec<Tile>
{
    let mut tiles: Vec<Tile> = vec![];

    let next = |time| (time+1) % valley.weather_maps;

    // move forward one unit in time in all possible directions
    match tile {
        Tile::Valley(time, row, col) => {
            // can we step south/north/east/west from here
            if *row < valley.height-1 { tiles.push(Tile::Valley(next(time), *row+1, *col  )) }
            if *row > 0               { tiles.push(Tile::Valley(next(time), *row-1, *col  )) }
            if *col < valley.width-1  { tiles.push(Tile::Valley(next(time), *row  , *col+1)) }
            if *col > 0               { tiles.push(Tile::Valley(next(time), *row  , *col-1)) }

            // we can enter the start/goal positions if we're in the exact right cell
            if *row == 0 && *col == 0 { tiles.push(Tile::Start(next(time))) }

            if    *row == valley.height-1
               && *col == valley.width-1 { tiles.push(Tile::Goal(next(time))) }
        },
        Tile::Start(time) => tiles.push(Tile::Valley(next(time), 0, 0)),
        Tile::Goal(time)  => tiles.push(Tile::Valley(next(time), valley.height-1, valley.width-1))
    }

    // move forward in time but stay put at the current position
    match tile {
        Tile::Valley(time, row, col) => tiles.push(Tile::Valley(next(time), *row, *col)),
        Tile::Start(time)            => tiles.push(Tile::Start(next(time))),
        Tile::Goal(time)             => tiles.push(Tile::Goal(next(time)))
    }

    // those were all the possible moves given where we were on the board. we still have to
    // account for the weather, so keep only the positions that are not blizzards
    tiles.into_iter()
         .filter(|tile| !valley[tile])
         .collect()
}

// construct a ValleyMap<bool> from the input, with true wherever/whenever there's a blizzard
fn valley_from_input(s: &str) -> ValleyMap<bool> {
    let lines   = s.lines().collect::<Vec<&str>>();
    let weather = Weather::from_strings(&lines[1..lines.len()-1]);
    let height  = weather.height;
    let width   = weather.width;

    assert!(height >= 1);
    assert!(width  >= 1);

    // the blizzards loop around forever and never turn, so a whole weather map
    // will repeat after width*height time. if we're lucky we'll have gcd > 1
    // which reduces our search space even further
    let gcd = num::integer::gcd(height, width);
    let weather_maps = height * width / gcd;

    // allocate a tile for each position in the valley plus the start and goal
    // positions, multiplied by the number of different weather pictures
    let tiles: Vec<bool> = vec![ false; weather_maps * (height * width + 2) ];

    let mut valley: ValleyMap<bool> = ValleyMap {
        tiles,
        height,
        width,
        weather_maps
    };

    // mark true when/where there's a blizzard. after these are set we don't have
    // to keep track of the direction or number of blizzards on a tile, just that
    // there was at least one blizzard there at that time
    for (r, row) in weather.tiles.iter().enumerate() {
        for (c, &tile) in row.iter().enumerate() {

            match tile {
                '>' => for time in 0..valley.weather_maps {
                           let index = Tile::Valley(time, r, (time + c) % width);
                           valley[&index] = true;
                       },
                '<' => for time in 0..valley.weather_maps {
                           let index = Tile::Valley(time, r, ((c + time*width) - time) % width);
                           valley[&index] = true;
                       },
                'v' => for time in 0..valley.weather_maps {
                           let index = Tile::Valley(time, (time + r) % height, c);
                           valley[&index] = true;
                       },
                '^' => for time in 0..valley.weather_maps {
                           let index = Tile::Valley(time, ((r + time*height) - time) % height, c);
                           valley[&index] = true;
                       },
                '.' => {},
                 w  => panic!("unknown weather {w}")
            }
        }
    }
    valley
}

struct Weather {
    tiles:  Vec<Vec<char>>,
    height: usize,
    width:  usize
}

impl Weather {
    /*  #.....#
        #>....#
        #.....#
        #...v.#
        #.....#
    */
    fn from_strings(s: &[&str]) -> Self {
        let tiles: Vec<Vec<char>> = s.iter()
                                     .map(|line| &line[1..line.len()-1])
                                     .map(|line| line.chars().collect())
                                     .collect();

        let width  = tiles[0].len();
        let height = tiles.len();

        Weather {
            tiles,
            width,
            height
        }
    }
}


/* Breadth-First Search for ValleyMap */

// start at the given tile and find the distances to all reachable tiles
fn bfs(valley    : &ValleyMap<bool>,
       neighbours: impl Fn(&Tile) -> Vec<Tile>,
       start     : Tile) -> ValleyMap<Option<usize>>
{
    // initialize map of distances
    let mut distance_to: ValleyMap<Option<usize>> = valley.to_initialized_distances();

    // set the first known distance: 0 from the start to the start
    distance_to[&start] = Some(0);

    // queue up the starting vertex
    let mut queue: Vec<Tile> = vec![start];
    let mut index = 0;

    while index < queue.len() {

        let u: Tile = queue[index].clone();
        index += 1;

        // get all vertices adjacent to this one that haven't been explored yet
        let neighbours: Vec<Tile> = neighbours(&u).into_iter()
                                                  .filter(|v| distance_to[v].is_none())
                                                  .collect();

        for v in neighbours {            
            distance_to[&v] = Some(distance_to[&u].unwrap() + 1);
            queue.push(v);
        }
    }

    distance_to
}

// zig-zag from start to goal, back to start, then back to goal again
fn part2(input: &Input) -> usize {
    zig_zag(&input.valley, 3, true, 0)
}

fn zig_zag(valley         : &ValleyMap<bool>,
           trips_left     : usize,
           direction      : bool,
           distance_so_far: usize) -> usize
{
    if trips_left == 0 {
        return distance_so_far
    }

    let distances = explore(valley,
                            if direction { Tile::Start(distance_so_far) }
                                    else { Tile::Goal (distance_so_far) } );

    let time = (0..valley.weather_maps)
        .into_iter()
        .map(|time| if direction { distances[&Tile::Goal (time)].unwrap() }
                            else { distances[&Tile::Start(time)].unwrap() } )
        .min()
        .unwrap();

    zig_zag(valley,
            trips_left - 1,
            !direction,
            distance_so_far + time)
}

fn explore(valley: &ValleyMap<bool>,
           start : Tile) -> ValleyMap<Option<usize>>
{
    let follow_edges = |tile: &Tile| get_neighbours(valley, tile);
    let distances = bfs(valley, follow_edges, start);

    distances
}


/* Imports */

use std::ops::{Index, IndexMut};


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            valley: valley_from_input(s)
        }
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_from_string() {
        let weather = Weather::from_strings(&["#.....#",
                                              "#>....#",
                                              "#.....#",
                                              "#...v.#",
                                              "#.....#"]);

        assert_eq!(weather.tiles.len(), 5);
        assert_eq!(weather.tiles[1][0], '>');
        assert_eq!(weather.tiles[1][1], '.');
        assert_eq!(weather.tiles[3][3], 'v');
        assert_eq!(weather.tiles[3].len(), 5);
    }

    #[test] fn test_part1() { assert_eq!(part1(&get_example()), 18); }
    #[test] fn test_part2() { assert_eq!(part2(&get_example()), 54); }

    #[test] fn test_part1_mine() { assert_eq!(part1(&Input::from("input.txt")), 290); }
    #[test] fn test_part2_mine() { assert_eq!(part2(&Input::from("input.txt")), 842); }

    #[test]
    fn test_valley_from_weather_map() {
        let valley = get_simple_valley();

        // east-moving blizzard
        assert!(!valley[&Tile::Valley(0, 1, 1)]); // there's not yet one to the east
        assert!(valley[&Tile::Valley(0, 1, 0)]);
        assert!(valley[&Tile::Valley(1, 1, 1)]);
        assert!(valley[&Tile::Valley(2, 1, 2)]);
        assert!(valley[&Tile::Valley(3, 1, 3)]);
        assert!(valley[&Tile::Valley(4, 1, 4)]);
        assert!(valley[&Tile::Valley(5, 1, 0)]); // time should wrap around to 0

        // west
        assert!(valley[&Tile::Valley(0, 2, 1)]);
        assert!(valley[&Tile::Valley(1, 2, 0)]);
        assert!(valley[&Tile::Valley(2, 2, 4)]);
        assert!(valley[&Tile::Valley(3, 2, 3)]);
        assert!(valley[&Tile::Valley(4, 2, 2)]);

        // south
        assert!(valley[&Tile::Valley(0, 3, 3)]);
        assert!(valley[&Tile::Valley(1, 4, 3)]);
        assert!(valley[&Tile::Valley(2, 0, 3)]);
        assert!(valley[&Tile::Valley(3, 1, 3)]);
        assert!(valley[&Tile::Valley(4, 2, 3)]);
        assert!(valley[&Tile::Valley(5, 3, 3)]);

        // north
        assert!(valley[&Tile::Valley(0, 0, 2)]);
        assert!(valley[&Tile::Valley(1, 4, 2)]);
        assert!(valley[&Tile::Valley(2, 3, 2)]);
        assert!(valley[&Tile::Valley(3, 2, 2)]);
        assert!(valley[&Tile::Valley(4, 1, 2)]);
        assert!(valley[&Tile::Valley(5, 0, 2)]);
    }

    fn get_example() -> Input {
        Input::from_string(
            "#.######\n\
             #>>.<^<#\n\
             #.<..<<#\n\
             #>v.><>#\n\
             #<^v^^>#\n\
             ######.#"
        )
    }

    fn get_simple_valley() -> ValleyMap<bool> {
        valley_from_input("#.#####\n\
                           #..^..#\n\
                           #>....#\n\
                           #.<...#\n\
                           #...v.#\n\
                           #.....#\n\
                           #####.#")
    }
}

/*  $ time target/release/day_24.exe
    Part 1: 290
    Part 2: 0

    real    0m0.159s
*/
