/*  https://adventofcode.com/2022/day/8  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { trees : Vec<Vec<u32>> }

// how many trees are visible from outside of the forest
fn part1(input: &Input) -> usize {
    let size = input.trees.len();

    // create a grid of bools with the same dimensions as the input, set all to false
    let mut visible: Vec<Vec<bool>> = vec![vec![false; size]; size];

    // left-to-right fly-overs
    for (row, vec) in input.trees.iter().enumerate() {
        let visibles = fly_over(vec.iter().copied());
        for (col, is_visible) in visibles.iter().enumerate() {
            visible[row][col] |= is_visible;
        }
    }

    // right-to-left
    for (row, vec) in input.trees.iter().enumerate() {
        let mut visibles = fly_over(vec.iter().rev().copied());
        visibles.reverse();
        for (col, is_visible) in visibles.iter().enumerate() {
            visible[row][col] |= is_visible;
        }
    }

    // top-down
    for col in 0..size {
        let visibles = fly_over(input.trees.iter().map(|row| row[col]));
        for row in 0..size {
            visible[row][col] |= visibles[row];
        }
    }

     // bottom-up
     for col in 0..size {
        let mut visibles = fly_over(input.trees.iter().map(|row| row[col]).rev());
        visibles.reverse();
        for row in 0..size {
            visible[row][col] |= visibles[row];
        }
    }

    visible.iter()
           .flat_map(|row| row.iter().filter(|col| **col))
           .count()
}

// see which trees are visible by flying over a line of them
fn fly_over(mut trees: impl Iterator<Item=u32>) -> Vec<bool> {

    // the first tree in a line is always visible
    let mut visible = vec![true];
    let mut highest = trees.next().expect("expected at least one tree in a line");

    // this for loop starts with the second height because the trees iterator already
    // passed over the first height when we called next() on it above
    for tree in trees {
        if tree > highest {
            visible.push(true);
            highest = tree;
        } else {
            visible.push(false);
        }
    }

    visible
}

// find the tree that sees the most other trees from its vantage point
fn part2(input: &Input) -> usize {
    let mut high_score = 0;

    for row in 0..input.trees.len() {
    for col in 0..input.trees.len() {
        let height = input.trees[row][col];

        let to_right = count_visible(input.trees[row].iter()
                                                     .skip(col+1)
                                                     .copied(),
                                     height);

        let to_left  = count_visible(input.trees[row].iter()
                                                     .take(col)
                                                     .rev()
                                                     .copied(),
                                     height);

        let to_down  = count_visible(input.trees.iter()
                                                .map(|row| row[col])
                                                .skip(row+1),
                                     height);

        let to_up    = count_visible(input.trees.iter()
                                                .map(|row| row[col])
                                                .take(row)
                                                .rev(),
                                     height);

        let scenic_score = to_right * to_left * to_down * to_up;

        high_score = high_score.max(scenic_score);
    }}

    high_score
}

// count the number of visible trees until our view is obstructed
fn count_visible(mut heights: impl Iterator<Item=u32>, from: u32) -> usize {
    let mut count = 0;

    loop {
        let height = heights.next();

        match height {
            Some(height) => {
                if height < from {
                    count += 1;
                } else {
                    return count + 1
                }
            },

            // we've run out of trees, the count must be all of them
            None => return count
        }
    }
}


impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(&contents)
    }

    fn from_string(lines: &str) -> Self {
        Input {
            trees: lines.trim()
                        .split('\n')
                        .map(Input::digits_to_vec)
                        .collect::<Vec<Vec<_>>>()
        }
    }

    // me: "Write a Rust function that converts a string of digits into a Vec<u32>"
    //
    // ChatGPT: Here is a possible implementation of a function that converts a string
    // of digits into a Vec<u32> in Rust:
    fn digits_to_vec(s: &str) -> Vec<u32> {
        // Convert the string into a vector of digits
        s.chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect()
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = get_example();
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part2() {
        let input = get_example();
        assert_eq!(part2(&input), 8);
    }

    #[test]
    fn test_fly_over() {
        assert_eq!(fly_over(vec![3,0,3,7,3].iter().copied()),
                   vec![true,false,false,true,false]);
    }

    fn get_example() -> Input {
        Input::from_string(
            "30373\n\
             25512\n\
             65332\n\
             33549\n\
             35390"
        )
    }
}
