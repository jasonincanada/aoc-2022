/*  https://adventofcode.com/2022/day/6  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { buffer: Vec<u8> }

// find the first index where the prior n chars are all distinct
fn part1(input: &Input) -> usize { marker_with_window_size(&input.buffer, 4)  }
fn part2(input: &Input) -> usize { marker_with_window_size(&input.buffer, 14) }

fn marker_with_window_size(buffer: &[u8], size: usize) -> usize {
    use std::collections::HashSet;

    let mut index = size;

    for window in buffer.windows(size) {
        // load the bytes from this window into a set, it will de-duplicate for us
        let set: HashSet<&u8> = HashSet::from_iter(window);

        // if the size of our set is the size of the window, all chars in it were distinct
        if set.len() == size {
            return index
        }

        index += 1;
    }

    0
}

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(string: &str) -> Self {
        Input {
            buffer: string.as_bytes()
                          .to_vec()
        }
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_with_window_size() {
        let input = Input::from_string("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(marker_with_window_size(&input.buffer, 4) , 7);
        assert_eq!(marker_with_window_size(&input.buffer, 14), 19);
    }
}
