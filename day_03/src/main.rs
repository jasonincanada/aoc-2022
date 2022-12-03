/*  https://adventofcode.com/2022/day/3  */

fn main() {
    let input = Input::from("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { rucksacks : Vec<Rucksack> }

struct Rucksack {
    items: String
}

impl Rucksack {
    fn from(line: &str) -> Self {
        Rucksack {
            items: line.to_string()
        }
    }

    // return slices of the left and right halves of this rucksack's items
    fn left_half(&self) -> &str {
        &self.items[0 .. self.items.len()/2]
    }

    fn right_half(&self) -> &str {
        &self.items[self.items.len()/2 ..]
    }

    // find the item common to the left and right halves of this rucksack
    fn common_item(&self) -> char {
        common_char(&[self.left_half(), self.right_half()])
    }

    // find the common item to a group of n >= 2 rucksacks
    fn common_item_group(group: &[Rucksack]) -> char {
        
        // collect references to the underlying strings in each Rucksack
        let strings: Vec<&str>
            = group.iter()
                   .map(|ruck| ruck.items.as_str())
                   .collect();

        common_char(&strings)
    }

    // each item has a priority as specified in the problem description
    fn priority(item: char) -> u32 {
        match item {
            'a'..='z' => item as u32 - 'a' as u32 + 1,
            'A'..='Z' => item as u32 - 'A' as u32 + 1 + 26,
             _        => panic!("item out of range")
        }
    }
}

// return the sum of priorities of the common types within each rucksack
fn part1(input: &Input) -> u32 {
    input.rucksacks.iter()
                   .map(Rucksack::common_item)
                   .map(Rucksack::priority)
                   .sum()
}

// return the sum of priorities of types common to groups of 3 rucksacks
fn part2(input: &Input) -> u32 {
    input.rucksacks.chunks(3)
                   .map(Rucksack::common_item_group)
                   .map(Rucksack::priority)
                   .sum()
}

// find the common char in a list of n >= 2 strings. this uses String::contains() so it's
// less efficient than using HashSets, but it seems simpler than the n-way hash intersections
// i found while googling. this code is adapted from: https://stackoverflow.com/a/65175232
fn common_char(group: &[&str]) -> char {
    assert!(group.len() >= 2);

    // start off with the chars from the first string
    let mut remaining : Vec<char> = group[0].chars().collect();

    // only retain chars that are found in all other strings
    remaining.retain(|item| {
        group[1..].iter()
                  .all(|string| string.contains(*item))
    });

    *remaining.first().unwrap()
}

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        let lines = contents.lines();

        Input {
            rucksacks: lines.map(Rucksack::from).collect()
        }
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack_left_right() {
        let rucksack = Rucksack::from("abcdef");
        assert_eq!(rucksack.left_half() , "abc");
        assert_eq!(rucksack.right_half(), "def");
    }

    #[test]
    fn test_rucksack_common_item() {
        let rucksack = Rucksack::from("abcb");
        assert_eq!(Rucksack::common_item(&rucksack), 'b');
    }

    #[test]
    fn test_rucksack_priority() {
        assert_eq!(Rucksack::priority('a'), 1);
        assert_eq!(Rucksack::priority('z'), 26);
        assert_eq!(Rucksack::priority('A'), 27);
        assert_eq!(Rucksack::priority('Z'), 52);
    }

    #[test]
    fn test_common_char() {
        assert_eq!(common_char(&["abc", "bde"       ]), 'b');
        assert_eq!(common_char(&["abc", "bde", "xyb"]), 'b');
    }
}
