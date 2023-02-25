/*  https://adventofcode.com/2022/day/15  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input, 2_000_000));
    println!("Part 2: {}", part2(&input, 4_000_000));
}

struct Input { sensors : Vec<Sensor> }

struct Sensor {
    pos   : Pos,
    beacon: Pos
}

struct Pos {
    x: i32,
    y: i32
}

// how many positions on a given row are covered by the sensor field
fn part1(input: &Input, row: i32) -> usize {
    row_coverage(&input.sensors, row)
}

fn row_coverage(sensors: &[Sensor], row: i32) -> usize {
    let mut intervals: Vec<Interval> = vec![];

    for sensor in sensors {
        if let Some(range) = get_x_range_on_y_for_sensor(row, sensor) {            
            // use our custom IntervalMerger iterator to merge this interval into the others
            // while keeping them all sorted and non-overlapping
            intervals = interval_merger(intervals.into_iter(), range)
                            .into_iter()
                            .collect();
        }
    }

    intervals.into_iter()
             .map(|interval| (interval.end() - interval.start() + 1) as usize)
             .sum::<usize>() - 1
}

// get the range of columns that this sensor's field intersects with on row y
fn get_x_range_on_y_for_sensor(y: i32, sensor: &Sensor) -> Option<Interval> {
    let dx = (sensor.pos.x - sensor.beacon.x).abs();
    let dy = (sensor.pos.y - sensor.beacon.y).abs();
    let manhattan = dx + dy;

    let row_dy = (sensor.pos.y - y).abs();

    let min_x = sensor.pos.x - (manhattan - row_dy);
    let max_x = sensor.pos.x + (manhattan - row_dy);

    if min_x <= max_x {
        Some(Interval::new(min_x, max_x))
    } else {
        None
    }
}

// find the only unaccounted-for coordinate in a square grid after rendering all sensor fields
fn part2(input: &Input, size: usize) -> usize {
    let pos = get_unaccounted_position(&input.sensors, size);

      pos.x as usize * 4_000_000
    + pos.y as usize
}

fn get_unaccounted_position(sensors: &[Sensor], size: usize) -> Pos {
    let mut rows: Vec<Vec<Interval>> = vec![vec![]; size + 1];
    let range = Interval::new(0, size as i32);

    for sensor in sensors {
        for row in row_range(sensor) {
            if !range.contains(&row) { continue }

            if let Some(range) = get_x_range_on_y_for_sensor(row, sensor) {
                rows[row as usize].push(range);
            }
        }
    }

    for (row_idx, row) in rows.into_iter().enumerate() {
        // remember to sort and merge the intervals
        let mut intervals: Vec<Interval> = vec![];

        for row in row.into_iter() {
            // use our custom iterator to merge this interval into the others
            intervals = interval_merger(intervals.into_iter(), row)
                            .into_iter()
                            .collect();
        }

        let gaps = get_gaps(intervals, &range);

        // there should only be one row with gaps.len() == 1
        if gaps.len() == 1 {
            return Pos {
                x: *gaps[0].start(),
                y:  row_idx as i32
            }
        }
    }

    Pos {x: 0, y:0}
}

// get the range of rows spanned by this sensor and its beacon
fn row_range(sensor: &Sensor) -> Interval {
    let dx = (sensor.pos.x - sensor.beacon.x).abs();
    let dy = (sensor.pos.y - sensor.beacon.y).abs();
    let manhattan = dx + dy;

    Interval::new(sensor.pos.y - manhattan,
                  sensor.pos.y + manhattan)
}


/* IntervalMerger Iterator */

type Interval = std::ops::RangeInclusive<i32>;

// our iterator maintains some mutable state to remember between next() calls
struct IntervalMerger<I: Iterator<Item=Interval>> {
    // the underlying iterator of Intervals. the intervals must be sorted by .start
    iter: I,

    // the new interval to add/merge into the outgoing stream of them
    new: Interval,

    // whenever we pull the next interval and it's non-overlapping to the right of
    // the interval we've been constructing, we suddenly have two on our hands: the
    // newly constructed one, and the next one that should come right after it.
    // but we can only return one Interval per call to next(), so here we queue
    // up the one we pulled too soon and it'll go out in the next call to next()
    queued: Option<Interval>,

    // true once we've returned the new interval
    returned: bool
}

impl<I> Iterator for IntervalMerger<I>
where
    I: Iterator<Item=Interval>
{
    type Item = Interval;

    fn next(&mut self) -> Option<Interval> {

        // if we queued up an interval in the last call, return it now
        if self.queued.is_some() {
            return self.queued.take()
        }

        // we've already returned the new interval so there's nothing left
        // to do but pass through the rest of them
        if self.returned { return self.iter.next() }

        // pull the next interval from the underlying iterator
        let next = self.iter.next();

        // none left, but we haven't returned the new one yet, so do it now
        if next.is_none() {
            self.returned = true;
            return Some(self.new.clone());
        }

        let next = next.unwrap();

        // pass through all the intervals non-overlapping to the left
        if next.end() < self.new.start() { return Some(next) }

        if self.new.end() < next.start() {
            self.queued = Some(next);
            self.returned = true;
            return Some(self.new.clone());
        }

        // the fun part, where we really get to benefit from the Iterator pattern.
        // we can keep calling next() on the underlying iterator even though we're
        // only planning to emit one Interval from this "outer" call of next()

        // XXX..XXXX...   underlying
        // ........XXXX   new
        let start = self.new.start().min(next.start());
        let end   = self.new.end()  .max(next.end());
        let mut new: Interval = Interval::new(*start, *end);

        loop {
            let next = self.iter.next();

            if next.is_none() {
                self.returned = true;
                return Some(new)
            }

            let next = next.unwrap();

            if new.end() < next.start() {
                self.queued = Some(next);
                self.returned = true;
                return Some(new);
            }

            // we don't need to consider the start here because the intervals were sorted,
            // meaning later intervals have larger starts than the one we're constructing
            let end = new.end().max(next.end());
            new = Interval::new(*start, *end);
        }
    }
}

// wrap an existing iterator of Intervals (already sorted by .start) to construct
// an iterator that merges a new Interval at the right spot in the underlying one
fn interval_merger<I>(iter: I, new: Interval) -> IntervalMerger<I>
where
    I: Iterator<Item=Interval>
{
    IntervalMerger {
        iter,
        new,
        queued: None,
        returned: false
    }
}

// take a list of intervals and a range, and return the gaps between the intervals
// constrained to the outer range
fn get_gaps(intervals: Vec<Interval>, range: &Interval) -> Vec<Interval> {

    if intervals.is_empty() {
        return vec![range.clone()]
    }

    let mut vec: Vec<Interval> = vec![];

    for window in intervals.windows(2) {
        let end   = *window[0].end();
        let start = *window[1].start();

        // a pair of intervals can be adjacent, there's no space between
        if end + 1 == start { continue }

        vec.push(Interval::new(end+1, start-1));
    }

    if intervals.is_empty() {
        vec.push(range.clone());
    }

    if range.start() < intervals[0].start() {
        vec.insert(0, Interval::new(*range.start(),
                                    intervals[0].start() - 1));
    }
    
    if range.end() > intervals.last().unwrap().end() {
        vec.push(Interval::new(intervals.last().unwrap().end() + 1,
                               *range.end()));
    }

    if vec.is_empty() { return vec }

    // drain anything at the left that is outside of the outer range
    while vec[0].end() < range.start() {
        vec.remove(0);
    }

    if vec.is_empty() { return vec }

    // still might have to edit the first interval
    vec[0] = Interval::new(*vec[0].start().max(range.start()),
                           *vec[0].end());

    if vec.is_empty() { return vec }

    // drain from the right
    while vec.last().unwrap().start() > range.end() {
        vec.remove(vec.len()-1);
    }

    if vec.is_empty() { return vec }
    
    let last = vec.last().unwrap();
    let last_idx = vec.len() - 1;

    // still might have to edit the last interval
    vec[last_idx] = Interval::new(*last.start(),
                                  *last.end().min(range.end()));

    vec
}


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            sensors: s.lines()
                      .map(Sensor::from_string)
                      .collect()
        }
    }
}

impl Sensor {
    // "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
    fn from_string(s: &str) -> Self {
        let numbers: Vec<i32> = extract_integers(s);

        Sensor {
            pos   : Pos { x: numbers[0], y: numbers[1] },
            beacon: Pos { x: numbers[2], y: numbers[3] }
        }
    }
}


/* AI Code */

// from my chat with ChatGPT: https://sharegpt.com/c/AtFTpwp
fn extract_integers(s: &str) -> Vec<i32> {
    let re = regex::Regex::new(r"-?\d+").unwrap();

    re.find_iter(s)
      .map(|m| m.as_str().parse().unwrap())
      .collect()
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&get_example(), 20), 26);
    }

    #[test]
    fn test_part2_example() {
       assert_eq!(part2(&get_example(), 20), 56000011);
    }

    #[test]
    fn test_part1_mine() {
        assert_eq!(part1(&Input::from("input.txt"), 2_000_000), 4883971);
    }

    #[test]
    fn test_part2_mine() {
       assert_eq!(part2(&Input::from("input.txt"), 4_000_000), 12691026767556);
    }

    #[test]
    fn test_interval_merger() {

        // the new interval is non-overlapping to the left of all the others
        let intervals = vec![Interval::new(3,5),
                             Interval::new(6,7)];
        let mut merged = interval_merger(intervals.into_iter(), Interval::new(1,2));

        assert_eq!(merged.next(), Some(1..=2));
        assert_eq!(merged.next(), Some(3..=5));
        assert_eq!(merged.next(), Some(6..=7));
        assert_eq!(merged.next(), None);

        // the new interval is non-overlapping to the right of all the others
        let intervals = vec![Interval::new(3,5),
                             Interval::new(6,7)];
        let mut merged = interval_merger(intervals.into_iter(), Interval::new(8,9));

        assert_eq!(merged.next(), Some(3..=5));
        assert_eq!(merged.next(), Some(6..=7));
        assert_eq!(merged.next(), Some(8..=9));
        assert_eq!(merged.next(), None);

        // the new interval fits in between two existing ones
        let intervals = vec![Interval::new(1,3),
                             Interval::new(6,7)];
        let mut merged = interval_merger(intervals.into_iter(), Interval::new(4,5));

        assert_eq!(merged.next(), Some(1..=3));
        assert_eq!(merged.next(), Some(4..=5));
        assert_eq!(merged.next(), Some(6..=7));
        assert_eq!(merged.next(), None);

        // the new interval overlaps two inner ones
        let intervals = vec![Interval::new(1,2),
                             Interval::new(4,5),
                             Interval::new(7,8),
                             Interval::new(9,10)];
        let mut merged = interval_merger(intervals.into_iter(), Interval::new(3,7));

        assert_eq!(merged.next(), Some(1..=2));
        assert_eq!(merged.next(), Some(3..=8));
        assert_eq!(merged.next(), Some(9..=10));
        assert_eq!(merged.next(), None);

        // the new interval engulfs all the others
        let intervals = vec![Interval::new(3,5),
                             Interval::new(6,7)];
        let mut merged = interval_merger(intervals.into_iter(), Interval::new(1,8));

        assert_eq!(merged.next(), Some(1..=8));
        assert_eq!(merged.next(), None);

        // the new interval is the only one
        let intervals = vec![];
        let mut merged = interval_merger(intervals.into_iter(), Interval::new(1,2));

        assert_eq!(merged.next(), Some(1..=2));
        assert_eq!(merged.next(), None);
    }

    #[test]
    fn test_get_gaps() {

        // list is empty
        let intervals = vec![];
        let gaps = get_gaps(intervals, &(1..=10));

        assert_eq!(gaps.len(), 1);
        assert_eq!(gaps[0], (1..=10));

        // outer interval is within the underlying interval total range
        let intervals = vec![Interval::new(3,5),
                             Interval::new(7,8),
                             Interval::new(11,13)];
        let gaps = get_gaps(intervals, &(3..=13));

        assert_eq!(gaps.len(), 2);
        assert_eq!(gaps[0], 6..=6);
        assert_eq!(gaps[1], 9..=10);

        // outer interval starts before and ends after the underlying intervals
        let intervals = vec![Interval::new(3,5),
                             Interval::new(7,8),
                             Interval::new(11,13)];
        let gaps = get_gaps(intervals, &(1..=15));

        assert_eq!(gaps.len(), 4);
        assert_eq!(gaps[0], 1..=2);
        assert_eq!(gaps[1], 6..=6);
        assert_eq!(gaps[2], 9..=10);
        assert_eq!(gaps[3], 14..=15);

        // outer interval starts and ends within the underlying interval total range
        let intervals = vec![Interval::new(3,5),
                             Interval::new(7,8),
                             Interval::new(11,13),
                             Interval::new(15,17)];
        let gaps = get_gaps(intervals, &(8..=13));

        assert_eq!(gaps.len(), 1);
        assert_eq!(gaps[0], 9..=10);

        // outer interval starts and ends in the midst of a gap
        let intervals = vec![Interval::new(3,5),
                             Interval::new(10,14),
                             Interval::new(19,20)];
        let gaps = get_gaps(intervals, &(8..=16));

        assert_eq!(gaps.len(), 2);
        assert_eq!(gaps[0], 8..=9);
        assert_eq!(gaps[1], 15..=16);

        // outer interval fully subsumes a single inner interval
        let intervals = vec![Interval::new(3,5)];
        let gaps = get_gaps(intervals, &(1..=7));

        assert_eq!(gaps.len(), 2);
        assert_eq!(gaps[0], 1..=2);
        assert_eq!(gaps[1], 6..=7);

        // outer interval is subsumed by a single inner interval
        let intervals = vec![Interval::new(1,5)];
        let gaps = get_gaps(intervals, &(2..=4));

        assert_eq!(gaps.len(), 0);
    }

    #[test]
    fn test_row_range() {
        let sensor = Sensor {
            pos   : Pos { x: 8, y:  7 },
            beacon: Pos { x: 2, y: 10 }
        };

        let range = row_range(&sensor);

        assert_eq!(*range.start(), -2);
        assert_eq!(*range.end()  , 16);
    }

    #[test]
    fn test_extract_integers() {
        assert_eq!(extract_integers("Sensor at x=12, y=14: closest beacon is at x=-10, y=16"),
                   vec![12, 14, -10, 16]);
    }

    #[test]
    fn test_get_x_range_on_y_for_sensor() {
        let sensor = Sensor {
            pos   : Pos { x: 8, y: 7  },
            beacon: Pos { x: 2, y: 10 }
        };

        assert_eq!(get_x_range_on_y_for_sensor(-2, &sensor), Some(8..=8));
        assert_eq!(get_x_range_on_y_for_sensor(-1, &sensor), Some(7..=9));
        assert_eq!(get_x_range_on_y_for_sensor( 7, &sensor), Some(-1..=17));

        // test out of the sensor's range
        assert_eq!(get_x_range_on_y_for_sensor(-3, &sensor), None);
        assert_eq!(get_x_range_on_y_for_sensor(17, &sensor), None);
    }

    fn get_example() -> Input {
        Input::from_string(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
             Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
             Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
             Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
             Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
             Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
             Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
             Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
             Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
             Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
             Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
             Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
             Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
             Sensor at x=20, y=1: closest beacon is at x=15, y=3"
        )
    }
}

/*  $ time target/release/day_15.exe
    Part 1: 4883971
    Part 2: 12691026767556

    real    0m2.653s
*/
