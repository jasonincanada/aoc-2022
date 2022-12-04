/*  https://adventofcode.com/2022/day/4  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { pairs : Vec<Pair> }

struct Pair {
    elf1: SectionRange,
    elf2: SectionRange
}

struct SectionRange {
    from: usize,
    to  : usize
}

impl SectionRange {
    fn fully_contains(&self, other: &SectionRange) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    // use bitwise and (&) to test for overlap
    fn overlaps_with(&self, other: &SectionRange) -> bool {
        let elf1 = range_to_int(self.from, self.to);
        let elf2 = range_to_int(other.from, other.to);

        elf1 & elf2 > 0
    }
}

//                                                    87654321
// turn the range 4-6 into the integer with bitstring 00111000
fn range_to_int(from: usize, to: usize) -> u128 {
    assert!(to   <= 128-1);
    assert!(from <= 128);
    assert!(from <= to);

       2_u128.pow(to   as u32 + 1) - 1
    - (2_u128.pow(from as u32    ) - 1)

    // or we could flick each bit on individually and sum them all:
    //
    //      (from..=to).map(|section| (2 as u128).pow(section as u32))
    //                 .sum()
}

// count number of pairs where one section range fully contains the other
fn part1(input: &Input) -> usize {
    input.pairs.iter()
               .filter(|pair|    pair.elf1.fully_contains(&pair.elf2)
                              || pair.elf2.fully_contains(&pair.elf1))
               .count()
}

// count number of pairs that overlap
fn part2(input: &Input) -> usize {
    input.pairs.iter()
               .filter(|pair| pair.elf1.overlaps_with(&pair.elf2))
               .count()
}


/* Parsing */

// 2-4,6-8
impl Pair {
    fn from(line: &str) -> Self {
        let mut elves = line.split(',');

        Pair {
            elf1: SectionRange::from(elves.next().unwrap()),
            elf2: SectionRange::from(elves.next().unwrap())
        }
    }
}

impl SectionRange {
    fn from(pair: &str) -> Self {
        let mut sections = pair.split('-');

        SectionRange {
            from: sections.next().unwrap().parse().unwrap(),
            to  : sections.next().unwrap().parse().unwrap()
        }
    }
}

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        let lines = contents.trim().split('\n');

        Input {
            pairs: lines.map(Pair::from).collect()
        }
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_contains() {
        assert!(SectionRange::from("2-8").fully_contains(&SectionRange::from("3-7")));
        assert!(SectionRange::from("4-6").fully_contains(&SectionRange::from("6-6")));
        assert!(! SectionRange::from("2-4").fully_contains(&SectionRange::from("6-8")));
        assert!(! SectionRange::from("5-7").fully_contains(&SectionRange::from("7-9")));
    }

    #[test]
    fn test_overlaps_with() {
        assert!(SectionRange::from("5-7").overlaps_with(&SectionRange::from("7-9")));
        assert!(SectionRange::from("2-8").overlaps_with(&SectionRange::from("3-7")));
        assert!(SectionRange::from("6-6").overlaps_with(&SectionRange::from("4-6")));
        assert!(SectionRange::from("2-6").overlaps_with(&SectionRange::from("4-8")));
        assert!(! SectionRange::from("2-4").overlaps_with(&SectionRange::from("6-8")));
        assert!(! SectionRange::from("2-3").overlaps_with(&SectionRange::from("4-5")));
    }

    #[test]
    fn test_range_to_bits() {
        assert_eq!(range_to_int(4,6), 16+32+64);
    }
}
