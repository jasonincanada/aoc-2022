Advent this year for me is a chance to improve my Rust skills. Below is a brief overview of new things I tried per day, sort of an evolution of my experience with Rust:

## Day 12 - Hill Climbing Algorithm

It was inevitable and it happened on Day 12: a shortest path algorithm! Fortunately I was already familiar with Dijkstra and ended up porting the [pseudocode on Wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode) to its Rust equivalent.  Part 2 took a couple minutes to run (in release config; in debug config, forget about it ever finishing on my desktop PC) by trying each starting position until the shortest path to the target was found.  I realized I could flip around the search from end to start, which brought the total runtime for both parts below 200ms.  Both runs of `dijkstra()` do exactly the same computation, so a pre-operation step might make sense, but it's fast enough this way that I don't need to clutter up the code by doing preprocessing before calling `part1()`/`part2()`.


## Day 11 - Monkey in the Middle

Second time passing a closure to a function (first was Day 7 in `at_path_do<F>` ), but getting a bit more complicated, as this one returns a `u64` value instead of just working on the mutable argument and not returning anything, as in Day 7.  This helps extract the differences between part 1 and part 2 which turned out to be a small operation on the worry level after its usual effect from the monkey's operation.

```rust
fn monkey_in_the_middle<F>(monkeys: &Vec<Monkey>,
                           rounds: usize,
                           custom_op: F) -> usize
    where F: Fn(u64) -> u64
{ ... }
```

Part 1 usage:

```rust
fn part1(input: &Input) -> usize {    
    monkey_in_the_middle(&input.monkeys,
                         20,
                         |worry| worry / 3)
}
```

Part 2 usage:

```rust
fn part2(input: &Input) -> usize {
    // let product_of_divisors = ... ;
    monkey_in_the_middle(&input.monkeys,
                         10_000,
                         |worry| worry % product_of_divisors)
}
```

Someone on reddit [was really happy](https://old.reddit.com/r/adventofcode/comments/zhjfo4/2022_day_10_solutions/izmqbuh/) about being able to use `strip_prefix()` for a prior day, so they were probably elated with today's parsing task, which would have been more elaborate without it. My first time using it:

```rust
impl Monkey {
    fn from_string(s: &str) -> Self {
        let lines: Vec<&str> = s.split('\n')
                                .map(|line| line.trim())
                                .collect();

        Monkey {
            items: lines[1].strip_prefix("Starting items: ").unwrap()
                           .split(", ")
                           .map(|item| item.parse().unwrap())
                           .collect(),

            operation   : Operation::from_expression(lines[2].strip_prefix("Operation: new = ").unwrap()),
            divisible_by: lines[3].strip_prefix("Test: divisible by "       ).unwrap().parse().unwrap(),
            if_true     : lines[4].strip_prefix("If true: throw to monkey " ).unwrap().parse().unwrap(),
            if_false    : lines[5].strip_prefix("If false: throw to monkey ").unwrap().parse().unwrap()
        }
    }
}
```


## Day 10 - Cathode-Ray Tube

Nothing interesting in this day except a few off-by-one errors I had to work around. A missed opportunity to use `abs_diff()` although my way way of using `[].contains()` is a bit more expressive anyway.


## Day 9 - Rope Bridge

After this one I really appreciated Rust's expressiveness and couldn't help thinking this might be a great language for a lot of reasons.  In `pull_rope_length` I used the following features:

- The function takes a ref to a slice instead of the vector (though it's a slice of the whole vector of course)
- The `vec![]` macro for easily constructing a certain number of the same object
- Calling `HashSet::new()` by qualifying its namespace in-place rather than using a separate `use` statement somewhere else
- Structure decomposition in the outer `for` loop so the fields are ready to go without having to assign them separately
- Using an underscore for the inner `for` loop index since it's not used anywhere, we're just doing something *n* times
- Using the `=` symbol to specify the range is inclusive, ie, don't leave out the last index `steps`
- Using `clone()` where it makes sense. In this case the HashSet takes ownership of the value passed to it, so I need to make a copy of it. I could be using references of course but for now I'm avoiding lifetimes

```rust
// count the number of coordinates the tail of the rope visits as it's pulled around a grid
fn pull_rope_length(moves: &[Move], length: usize) -> usize {

    // start the whole rope bunched up on the 0,0 coordinate
    let mut rope: Vec<Coord> = vec![Coord {row:0,col:0}; length];

    // save the location of the tail after every move
    let mut visited = std::collections::HashSet::new();
    visited.insert(Coord {row:0,col:0});

    for Move {direction, steps} in moves.iter() {
        for _ in 1..=*steps {

            // move the first knot by one step and catch the rest up
            match direction {
                'R' => { rope[0].col += 1; deslackify(&mut rope); },
                'L' => { rope[0].col -= 1; deslackify(&mut rope); },
                'U' => { rope[0].row += 1; deslackify(&mut rope); },
                'D' => { rope[0].row -= 1; deslackify(&mut rope); },
                 _  =>   panic!("Unexpected direction")
            }

            // remember the location of the tail after this step
            visited.insert(rope[length-1].clone());
        }
    }

    visited.len()
}
```

My `deslackify()` function is a bit funny now that I've reviewed other participants' solutions and found the `signum()` function.


## Day 8 - Treetop Tree House

As I noted on the [sub-reddit](https://old.reddit.com/r/adventofcode/) for this day, it was *iterator-fu* night at the code dojo. There were many directions of iteration over the vectors of tree heights so lots of calisthenic practice here with those.  And something new for me: in Rust, iterators maintain mutable state and this enables flexible use of them in different ways in your code. See the comment on the `for` loop here:

```rust
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
```

This was the first day (first time ever actually) that I've used AI-generated code, and by simply asking it a question:

```rust
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
```

Crazy! Let's not tell the boss about this advancement. It takes all day to type this stuff out you know


## Day 7 - No Space Left on Device

I loved this one! First time using a `struct` that refers back to itself when modeling a directory (which can of course contain more directories):

```rust
struct Directory {
    name : String,
    dirs : Vec<Directory>,
    files: Vec<File>,
    total_size: usize
}
```

This was after I spent an hour or so trying to figure out how to use tree cursors using the [trees crate](https://crates.io/crates/trees). I gave up fighting the borrow checker and hand-coded a generic, recursive updating function `at_path_do()` that narrows down a path to a specific sub-directory and runs a mutable operation on it. I don't get the *O(1)* operations I was looking for with cursor movements, but each node insert operation is still *O(log n)*† which you'd expect from a general tree insert anyway. First time passing a closure to a function, hence the `F` trait bound below:

```rust
// recursively descend through the filesystem to a certain path,
// then do some operation on that directory
fn at_path_do<F>(node: &mut Directory,
                 path: &[String],
                 operation: F)
    where F: Fn(&mut Directory)
{
    // we're at the target path
    if path.is_empty() {
        // this will either add a file or a directory depending on how
        // it was defined in the calling code
        operation(node);
    } else {
        // recurse into the next directory in the path
        at_path_do(node.dirs.iter_mut()
                            .find(|dir| dir.name == path[0])
                            .expect("path contained a non-existent directory"),
                   &path[1..],
                   operation);
    }
}
```

To use it, for example to add a directory to an existing directory:

```rust
// a directory
if line.starts_with("dir ") {
    at_path_do(&mut root,
               &path,
               |node| node.dirs.push(Directory::new(line[4..].to_string())));
}
```

Another step requires us to tally up the sizes from the leaf nodes upwards, so I used `iter_mut()` for the first time:

```rust
// recursively sum the size of directories based on their files and sub-dirs sizes
fn tally_sizes(node: &mut Directory) -> usize {

    node.total_size =
        node.dirs.iter_mut()        // this needs to be iter_mut() because within map's call to
                 .map(tally_sizes)  // tally_sizes the node.total_size field will be updated
                 .sum::<usize>()

      + node.files.iter()           // add the total file sizes from this directory
                  .sum::<usize>();

    node.total_size
}
```

My experience recursing over trees in Haskell probably helped me write this code a bit more smoothly. It resembles the recursive application of an algebra to fold a tree down to its sum, but it keeps the tree in place and even mutably updates it along the way. Very cool!

Part 2 requires a sort, as in day 1, in order to do something starting from one end of a sorted list:

```rust
let mut totals = get_all_totals_from(&input.system);

totals.sort();
totals.into_iter()
      .skip_while(|&size| size < free_at_least)
      .next()
      .expect("assumed at least one size >= free_at_least")
```

By now I'm wondering if it's still smart to call `.sort()` to mutably sort an object in place before doing anything more on it, or just adding `.sorted()` or `.sort_by()` to the start of an iterator chain. For now I like the idea of practicing switching between in-place updates and immutable chaining, so I leave the sorting step out of the chain.  It's an *O(n log n)* step either way.

Also, first use of `into_iter()` instead of just `iter()`. This makes sense at the end of functions when the vec being iterated over is about to be dropped anyway; you might as well take ownership of the values and be able to use them directly without having to dereference them with `*`. This isn't as obvious in the above code snippet, since `skip_while` hands a reference to its closure, compared to calling `.map()` after `.into_iter()`, as in Day 12:

```rust
positions_of('a', &input.heightmap)
    .into_iter()
    .map(|pos| distances[pos.row][pos.col])  // this pos is Pos instead of &Pos because we used .into_iter()
    .min().unwrap()
```

On this day I started testing the example inputs in the unit test section, `test_part1()`/`test_part2()`

(† in spirit anyway, since I use a `Vec` for holding the directories and they still require an *O(n)* search to locate the next one in the path)


## Day 6 - Tuning Trouble

Easy day, nothing new except maybe putting `use std::collections::HashSet` not at the very top of the file but much closer to its actual use, in this case at the top of the `marker_with_window_size()` function, since that's the only function that uses a HashSet.  First use of the `.windows()` function on slices.


## Day 5 - Supply Stacks

We need to mutate the heck out of a bunch of vectors that represent stacks of crates in a shipyard. They are parsed into a read-only `Input` object however, so on this day I started thinking about mutation strategies. For today I cloned the ship out of the input (so, not cloning the whole input, which would have duplicated the move list `Input::moves` unnecessarily since it never changes) and then worked on that in-place, rather than mapping over existing vectors in the process of creating a bunch of new ones for each move.

First time using Rust's built-in `Vec::split_off()` function while grabbing a bunch of crates. This function appropriately models what's actually happening; I actually choose `grab` as the variable name because it's holding the vec of crates it just grabbed.


## Day 4 - Camp Cleanup

Removed the blank line in `main` between reading the input and processing the parts. This is the start of slowly squeezing out the vertical whitespace in my code:

```rust
fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
```

I noticed the session IDs in the ranges are always under 128, so to calculate the intersection between two ranges, I flicked on bits and used a bitwise AND:

```rust
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
```

Not unique to Rust really but I thought it was a bit clever.

The domain was modelled in a somewhat object-oriented format (see the `SectionRange` impl) but this is the last time I did so, at least in the first 12 days.


## Day 3 - Rucksack Reorganization

Returning slices of a string instead of cloning to a new string:

```rust
// return slices of the left and right halves of this rucksack's items
fn left_half(&self) -> &str {
    &self.items[0 .. self.items.len()/2]
}
```

Used `Vec::retain` on a mutable vector:

```rust
// only retain chars that are found in all other strings
remaining.retain(|item| {
    group[1..].iter()
              .all(|string| string.contains(*item))
});
```


## Day 2 - Rock Paper Scissors

First use of enums to model the problem domain, in this case a basic Rock Paper Scissors game with three possible outcomes per round.  As well, a trick to avoid qualifying the enum name itself every time one of the variants is used:

```rust
// so we don't have to keep writing Shape:: and Outcome:: in front of the enums
use crate::Shape::*;
use crate::Outcome::*;
```

First time using multiple possibilities in `match` arms:

```rust
match letter {
    'A' | 'X' => Rock,
    'B' | 'Y' => Paper,
    'C' | 'Z' => Scissors,
     _        => panic!("Unexpected letter")
}
```

Rust does point-free function name passing to `map()`:

```rust
input.rounds.iter()
            .map(points_for_part_1)
            .sum()
```


## Day 1 - Calorie Counting

Here we go! The main input-handling strategy is to parse the input into a read-only `Input` structure, so we can pass an identical copy of it to both parts. As I did with my C# solutions last year, I'll try to express `part1`/`part2` in terms of a common function that is parameterized just enough to show the differences between what part 1 and part 2 are doing.  For day one, after refactoring part 1 now that I know what part 2 is about, the difference is clear: the number of items to take off the top when calculating the sum is 1 in part 1 and 3 in part 2.

While parsing I used `.expect()` after `fs::read_to_string()` instead of `.unwrap()` to provide a more meaningful error in case the unwrap fails.
