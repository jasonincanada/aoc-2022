/*  https://adventofcode.com/2022/day/16  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { valves : Vec<Valve> }

// valves will be stored in a sparse vector by converting their two-letter
// names into indexes, so don't bother storing the names
struct Valve {
    index    : usize,
    flow_rate: u32,
    tunnels  : Vec<usize>
}

// a square grid (again, sparse) of shortest distances between any two valves
type DistanceGrid = Vec<Vec<u32>>;

// strategy:
//   build a square grid of shortest distances (using dijkstra) between any two non-zero valves
//      (my input has only 15 non-zero flow-rates)
//   depth-first search of all possible paths we have time to visit. it shouldn't be a complexity
//      disaster because with only 30 minutes to simulate we won't have time to visit all 15! leaves
fn part1(input: &Input) -> u32 {

    // build a square grid of shortest distances from all valves to all other valves
    let distances: DistanceGrid = get_distance_grid(&input.valves);

    // collect the starting list of valves with non-zero flow rates
    let closed_valves: Vec<&Valve> = input.valves.iter()
                                                 .filter(|valve| valve.flow_rate > 0)
                                                 .collect();

    let start = Valve::index_from("AA");
    let total_pressure = best_path(&distances, closed_valves, start, 30);

    total_pressure
}

// depth-first search through the whole tree of navigation possibilites. each leaf node
// represents a unique way to visit all closed non-0-flow-rate valves at least once
fn best_path(distances     : &DistanceGrid,
             closed_valves : Vec<&Valve>,
             at_valve      : usize,
             minutes_left  : u32) -> u32
{
    let mut pressures: Vec<u32> = vec![];

    // consider each of the closed valves as the next possible destination
    for valve in &closed_valves {
        let distance = distances[at_valve][valve.index];

        // can we even cover that distance with the amount of time we have left?
        if distance >= minutes_left { continue }

        // calculate the contribution of this valve being turned on for the time left available.
        // -1 because it takes a minute to turn the valve on before it starts flowing
        let pressure = valve.flow_rate * (minutes_left - distance - 1);

        // recurse on the rest of the unopened valves after removing this one
        let remaining = closed_valves.iter()
                                     .filter(|v| v.index != valve.index)
                                     .cloned()
                                     .collect();

        let best_pressure = pressure + best_path(distances,
                                                 remaining,
                                                 valve.index,
                                                 minutes_left - distance - 1);

        pressures.push(best_pressure);
    }

    *pressures.iter()
              .max()
              .unwrap_or(&0)
}

// compute the shortest distances from each valve to every other valve. call dijkstra
// once for each valve so it can fan out from that valve as the starting position
fn get_distance_grid(valves: &[Valve]) -> DistanceGrid {
    let last_index = Valve::index_from("ZZ");

    let mut edges   : Vec<Vec<usize>> = vec![ vec![]; last_index+1 ];
    let     vertices: Vec<usize>      = valves.iter()
                                              .map(|valve| valve.index)
                                              .collect();

    // build the edges list
    for valve in valves {
        for tunnel in &valve.tunnels {
            edges[valve.index].push(*tunnel);
        }
    }

    let mut grid: DistanceGrid = vec![ vec![0; last_index+1]; last_index+1 ];

    // run dijkstra for each valve with that valve as the starting point of the search
    for valve in valves {
        let distances = dijkstra(&edges, &vertices, last_index, valve.index);
        grid[valve.index] = distances;
    }

    grid
}

// this time we have an elephant to help us. it takes 4 minutes to train him but then
// he can go around opening valves too
fn part2(_input: &Input) -> u32 {
    0
}


/* Dijkstra */

// Dijkstra's shortest path algorithm from the pseudocode on Wikipedia:
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
//
// start at the given valve and find the shortest path to all reachable valves
fn dijkstra(edges     : &[Vec<usize>],
            vertices  : &[usize],
            last_index: usize,
            start     : usize) -> Vec<u32>
{
    // initialize grid of "infinite" distances
    let mut distance_to: Vec<u32> = vec![ u32::MAX-1; last_index+1 ];

    // queue up every coordinate
    use std::collections::HashSet;
    let mut queue: HashSet<usize> = vertices.iter().cloned().collect();

    // set the first known distance: 0 from the start to the start
    distance_to[start] = 0;

    while !queue.is_empty() {

        // find the position in the queue with shortest distance from the starting valve
        let u = *queue.iter()
                      .min_by(|&&a, &&b| distance_to[a].cmp(&distance_to[b]))
                      .unwrap();

        queue.remove(&u);

        // get all valves adjacent to the starting one that are still in the queue
        let neighbours: Vec<usize> = edges[u].iter()
                                             .filter(|valve| queue.contains(valve))
                                             .cloned()
                                             .collect();

        for v in neighbours {
            // a step to a neighbouring valve is always a distance of 1 (otherwise
            // we would've had to pass in edge weights to this function as well)
            let alt = distance_to[u] + 1;

            if alt < distance_to[v] {
                distance_to[v] = alt;
            }
        }
    }

    distance_to
}


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            valves: s.lines()
                     .map(Valve::from_string)
                     .collect()
        }
    }
}

impl Valve {
    // "Valve BB has flow rate=13; tunnels lead to valves CC, AA"
    fn from_string(s: &str) -> Self {
        let re = regex::Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)$").unwrap();
        let captures = re.captures(s).unwrap();

        Valve {
            index    : Valve::index_from(&captures[1]),
            flow_rate: captures[2].parse().unwrap(),
            tunnels  : Valve::tunnels_from(&captures[3])
        }
    }

    // "AA" -> 0
    // "AB" -> 1
    // "BB" -> 26
    fn index_from(s: &str) -> usize {
        let chars = s.as_bytes();
        (chars[0] - b'A') as usize * 26 + (chars[1] - b'A') as usize
    }

    // "AA, BC, DE"
    fn tunnels_from(s: &str) -> Vec<usize> {
        s.split(", ")
         .map(Valve::index_from)
         .collect()
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
        assert_eq!(part1(&get_example()), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_example()), 1707);
    }
    
    #[test]
    fn test_parse_valve() {
        let valve = Valve::from_string("Valve BB has flow rate=13; tunnels lead to valves CC, AA");
        assert_eq!(valve.index, 27);
        assert_eq!(valve.flow_rate, 13);
        assert_eq!(valve.tunnels[0], Valve::index_from("CC"));
        assert_eq!(valve.tunnels[1], Valve::index_from("AA"));

        // how cheeky, the input is gramatically correct so we need to test for a single tunnel
        let valve = Valve::from_string("Valve HH has flow rate=22; tunnel leads to valve GG");
        assert_eq!(valve.index, 189);
        assert_eq!(valve.flow_rate, 22);
        assert_eq!(valve.tunnels[0], Valve::index_from("GG"));
    }

    #[test]
    fn test_index_from() {
        assert_eq!(Valve::index_from("AA"), 0);
        assert_eq!(Valve::index_from("AB"), 1);
        assert_eq!(Valve::index_from("BA"), 26);
    }

    fn get_example() -> Input {
        Input::from_string(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
             Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
             Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
             Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
             Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
             Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
             Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
             Valve HH has flow rate=22; tunnel leads to valve GG\n\
             Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
             Valve JJ has flow rate=21; tunnel leads to valve II"
        )
    }
}
