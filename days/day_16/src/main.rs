/*  https://adventofcode.com/2022/day/16  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { valves : Vec<Valve> }

#[derive(Debug, PartialEq)]
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
// he can go around opening valves too. this may seem really complicated: how do we
// simultaneously keep track of both us and the elephant and make sure we've considered
// all possible ways for both of us to navigate?
//
// but it's pretty easy: since we don't have to visit a valve that's been opened by
// the elephant, and vice versa, the best solution will be one where we visited a certain
// set of valves, and the elephant visited valves from the *complement* of our set. so
// the problem is now to simulate visiting all possible subsets of valves, then adding up
// the various complements to find the maximum possible pressure from us working together
fn part2(input: &Input) -> u32 {

    // build the same square grid as in part 1 of shortest distances between all valves
    let distances: DistanceGrid = get_distance_grid(&input.valves);

    // get the non-zero flow rate valves only
    let valves = input.valves.iter()
                             .filter(|v| v.flow_rate > 0)
                             .collect::<Vec<&Valve>>();

    let count = valves.len();

    // our meagre RAM only holds so much and our CPU is only so quick.
    // let's allocate for no more than 2^16 valve sets
    assert!(count <= 16);

    let last_index = 2_u32.pow(count as u32);

    // phase 1: compute and store the result of best_path() for each possible subset of valves
    let mut pressures: Vec<u32> = vec![ 0; last_index as usize ];
    
    for i in 1..last_index {
        let valve_set: Vec<&Valve> = get_valves_for_bitstring(i, count, &valves);
        let pressure = best_path(&distances,
                                 valve_set,
                                 Valve::index_from("AA"),
                                 30-4); // subtract 4 minutes to train the elephant

        pressures[i as usize] = pressure;
    }

    // phase 2: find the best pressure possible when adding the pressure from one set of
    // valves to its *complement* set of valves. this accounts for both us and the elephant
    let mut best = 0;

    for us in 1..last_index {
        let elephant = bitstring_complement(us, count as u32);
        let sum = pressures[us as usize]
                + pressures[elephant as usize];

        best = best.max(sum)        
    }

    best
}

// if the bitstring is 5 (binary 101) then include the 1st and 3rd valves
fn get_valves_for_bitstring<'a>(bitstring: u32,
                                count    : usize,
                                valves   : &[&'a Valve]) -> Vec<&'a Valve>
{
    get_elements_for_bitstring(bitstring, count, valves)
}

fn get_elements_for_bitstring<'a, T>(
    bitstring: u32,
    count    : usize,
    elements : &[&'a T]
) -> Vec<&'a T>
{
    let mut vec: Vec<&T> = Vec::with_capacity(count);

    for (i, el) in elements.iter().enumerate() {
        let anded = bitstring & 2_u32.pow(i as u32);
        if anded > 0 {
            vec.push(el);
        }
    }

    vec
}

// 011000 -> 100111
fn bitstring_complement(num: u32, bit_count: u32) -> u32 {
    !(num as u32) & (2_u32.pow(bit_count)-1)
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
    fn test_part1() {
        assert_eq!(part1(&get_example()), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_example()), 1707);
    }

    #[test]
    fn test_get_valves_for_bitstring() {
        let valve0 = Valve { index: 0, flow_rate: 1, tunnels: vec![] };
        let valve1 = Valve { index: 1, flow_rate: 2, tunnels: vec![] };
        let valve2 = Valve { index: 2, flow_rate: 3, tunnels: vec![] };
        let valve3 = Valve { index: 3, flow_rate: 4, tunnels: vec![] };
        let valves = vec![ &valve0, &valve1, &valve2, &valve3 ];
        
        let ret = get_valves_for_bitstring(0, valves.len(), &valves);
        assert_eq!(ret.len(), 0);

        let ret = get_valves_for_bitstring(1, valves.len(), &valves);
        assert_eq!(ret.len(), 1);
        assert_eq!(ret[0], &valve0);

        let ret = get_valves_for_bitstring(2, valves.len(), &valves);
        assert_eq!(ret.len(), 1);
        assert_eq!(ret[0], &valve1);

        let ret = get_valves_for_bitstring(3, valves.len(), &valves);
        assert_eq!(ret.len(), 2);
        assert_eq!(ret[0], &valve0);
        assert_eq!(ret[1], &valve1);

        let ret = get_valves_for_bitstring(4, valves.len(), &valves);
        assert_eq!(ret.len(), 1);
        assert_eq!(ret[0], &valve2);
    }

    #[test]
    fn test_bitstring_complement() {
        assert_eq!(bitstring_complement(0, 4), 0b1111);
        assert_eq!(bitstring_complement(1, 4), 0b1110);
        assert_eq!(bitstring_complement(2, 4), 0b1101);
        assert_eq!(bitstring_complement(15, 4), 0b0000);
        assert_eq!(bitstring_complement(14, 4), 0b0001);
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

/*  $ time target/release/day_16.exe
    Part 1: 2119
    Part 2: 2615

    real    0m10.727s
*/
