Advent this year for me was a chance to improve my Rust skills. Below is a brief overview of new things I tried per day, sort of an evolution of my experience with Rust:

## Day 25 - Full of Hot Air

Hey man, I don't work on Jesus' birthday, so I didn't do today's puzzles. That and I couldn't figure out how to convert back from decimal to SNAFU format, thanks to the negative multiples of five throwing a wrench into the gears, so I managed to finish neither part today.


## Day 24 - Blizzard Basin

This was my favourite day this year and the solution I'm most happy with even though it's slightly over-engineered. My first attempt after reading the description was the obvious naive one: track where each blizzard is at all times, mutably stepping them across the board as I explore the movement decision tree. I noticed early on that the blizzard maps repeat after `width*height` iterations since their trajectories are perfectly straight and they wrap around predictably at the ends of the valley. So my first attempt at making it more efficient was to compute all the valleys ahead of time and loop through the pre-computed ones while exploring the tree.  I stored the main map as a `Vec<Vec<Vec<char>>>` but before long there was obvious code smell, with functions like this one:

```rust
fn is_wall(tile: &Vec<char>) -> bool {
    tile.len() == 1 && tile[0] == WALL
}
```

I started thinking functionally about the problem, and what benefits I could get from a more immutable approach.  A separation of concerns turned out to be a smart way to think about the problem, and before long I had `ValleyMap<T>` figured out as the smallest data structure that could be used to store all positions in a valley at all times. The generic `<T>` type parameter allows the separation of the problem into two distinct conceptual phases: computing which cells at which times have no blizzards (`T ~ bool`), which is done during parsing; then using the same shape of valley to store the distance from the start to every reachable position (`T ~ Option<usize>`) with an fmap-like operation to copy the shape but change all the tiles to `None`:

```rust
// copy the shape of the valley and set all tiles to None (distance not yet computed)
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
```

As a bonus this gave me the opportunity to implement a pair of Rust traits: `Index` and `IndexMut`. This type of scenario is exactly what they're for: setting up a custom indexing system into a custom data structure that is more complicated than a simple offset into a vector. This allows succinct structure updates at a specific position using normal indexing syntax, as in the distance-updating part of `bfs()`:

```rust
for v in neighbours {
    distance_to[&v] = Some(distance_to[&u].unwrap() + 1);
    queue.push(v);
}
```

The index uses a custom sum type (enum) `Tile` that captures the three types of positions, with the added time dimension running off into the distance for all three.

```rust
// a tile is an index into our 3D grid (two space and one time). Start/Goal represent the
// fixed start and goal positions, Valley is somewhere on the main grid
enum Tile {
    Valley(usize, usize, usize), // time, row, col
    Start(usize),                // time
    Goal(usize)                  // time
}
```

Until this separation I was bending over backwards to treat the start and goal positions like their own x/y coordinates on a larger grid. At first I even shifted the grid down and added a `#` above the start position so it would look like any other wall that the stepping logic would run into and ignore. After the separation the start and goal positions are now logical ideas, not physical ones, and the overall solution in my view is much cleaner.

A second benefit of the immutability approach was obvious when considering the blizzards and how to store where they were at what times. There is now no concept of mutably removing a blizzard from one cell and pushing it onto another. Since the grid goes off in an orthogonal time direction, and the blizzards all loop around after `width*height`, we only have to spray each blizzard individually across each unit of time, wrapping accordingly. This way we never have to track a bunch of blizzards on one tile at one time: just that there was any blizzard on a tile at that exact time. This de-smells the code significantly; in particular, we only use each blizzard's ASCII representation (`>`, `<`, `v`, `^`) exactly once in the file, in the `valley_from_input()` function, taking care of the "moving" of the blizzards in one swoop:

```rust
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
```

Part two looked at first like it could pose a big challenge. We have to zig-zag back and forth 3 trips across the valley. Is it as simple as I hope? Can we just run three separate `bfs()` searches using the end of one round as the stepping-off point for the next round? It turns out we can. I was worried there would be an overall benefit to taking a less-than-optimal route through the valley in one direction if it enabled an even quicker trip in the opposite direction by the time the weather was all taken into account. But it seems--at least for my input--that the simpler idea to just run as fast as possible all three ways works fine, and it answers the second part successfully.

Even with the advent of `ValleyMap<T>` and its immutable character, it still took a really long time to finish even the first part. I added some print statements to get a report of how many of the 650k starting vertices were left for dijkstra to check, and it was dwindling down slowly enough that I took a couple hour nap and woke up some time after it finished.  It hit me that there might be a quicker way to do the search given that the edge weights were all the same 1 distance. I [confirmed this idea with ChatGPT](https://sharegpt.com/c/bzC8YtK) and switched from dijkstra to a much faster breadth-first search algorithm. This pulled a nasty `O(n log n)` step out of the inner search loop, and all said and done, both parts now finish in about half a second instead of a few hours.

Over the weeks of this competition, ChatGPT's interface had a couple improvements, most notably that your questions and answers are stored in a table-of-contents on the left hand side instead of disappearing forever once you closed the tab.  It even smartly titles your queries for you; in the case of the above: "Dijkstra vs BFS Comparison".  I still give it the suspicious eye though, since the internals itself are probably no better than they were at the start of the month: worthy of heavy skepticism and not to be used seriously unless you have the aptitude to think critically about its responses.


## Day 23 - Unstable Diffusion

Finally the year's cellular automata puzzle. The rules are easy to understand and code so this one wasn't very difficult despite being so late in the year. Based on the sub-reddit's discussions for this day it looks like I took the relatively uncommon path of representing the data in a `Vec<Vec<char>>` instead of a hashset of positions or some other structure. This gave me constant-time updates of the elf positions, but left me with the tiresome concern for out-of-bounds indexing errors, since the vectors are 0-based and length-limited, while the elves are meandering around seemingly at random. However, knowing they can only move a maximum of one distance per turn, at the beginning of each turn I apply the slightly wasteful step of wrapping the whole grid in a rectangle of spaces:

```rust
impl Grove {
    // wrap a rectangle of empty tiles around the grid. called at the start of
    // every round to make sure there's enough space to index within bounds
    fn wrap_with_ground_tiles(&mut self) {
        for row in self.grid.iter_mut() {
            row.insert(0, GROUND);
            row.push(GROUND);
        }

        self.grid.insert(0, vec![ GROUND; self.grid[0].len()]);
        self.grid.push(     vec![ GROUND; self.grid[0].len()]);
    }
}
```

The available space will stay just ahead of the most expansive wandering efforts by all the elves. Other than that, a technically easy day with no real opportunities to find anything clever or innovative to do.


## Day 22 - Monkey Map

Easy first part, very difficult second part. I spent a couple nights unfolding and flipping 3D cubes in my head and couldn't figure out how to make it general across all possible cube unfoldings instead of just the sample and my input, which I could have manually marked up with a relatively small amount of effort.

- *Okay:* Hand-code both
- *Better:* Hand-describe the edge mapping
- *Betterer:* Hand-describe using the smallest amount of information, automatically derive the rest
- *Best:* Fully general unfolding while tracking edge mappings

I had this hilarious exchange with ChatGPT about cube diagrams: https://sharegpt.com/c/QkFg0n4. I've grown weary of using ChatGPT for anything technical lately; the first couple code clips I got early in the challenge were correct, but it's been off the mark with many other things. It's confidently wrong a lot of the time, which is good for a laugh in a casual competition like this, but should serve as a warning not to trust it blindly for anything important. It's great for creative prompts (a brilliant solution to writer's block, I'm sure authors will discover), and casual banter, but its technical solutions need to be met with a good deal of critical thinking.


## Day 21 - Monkey Math

Recursive expression tree parsing with a twist: in part 2, one of the leaf nodes is labelled `humn` and we need to determine what it would have to be for the two main subtrees off the root node to evaluate to the same thing, assuming the root operation was `==`.  I didn't realize we could just simplify the overall equation into a `y = mx + b` format and solve it easily from there (according to the reddit sub), but it's a good thing because I could finally do something non-trivial with function composition instead.  I decided to fold up the same tree given by the input for part one, but instead of rolling up the constant resulting from every sub-expression (as in part 1), one of the leaf nodes is `x`, the unknown. So with the `ResultOfDescent` type I compose a gradually growing function that will carry out the operations inverse to the one at each node. The end result at the top of the tree is a constant from one half, a function from the other half, and finding the answer amounts to simply calling the function on the constant.

Rust's [match construct](https://doc.rust-lang.org/book/ch06-02-match.html) really shows its strength today as it forces us to consider all possible arms, which in our case involves permutations of our sum types `Job` (part 1) and `ResultOfDescent` (part 2). This forces us to consider what would occur to get us to each arm, and gives us the opportunity to document any failure cases in meaningful terms using the `panic!` macro:

```rust
// in part 2, consider root's operation to be == and calculate what the "humn:" leaf
// node would have to yell out to make root's two sub-expressions equal
fn part2(input: &Input) -> i64 {

    match input.jobs.get("root").unwrap() {
        // evaluate both of root's sub-expressions, one must evaluate to a constant and
        // the other a function that takes that constant and returns what 'humn' must be
        Job::Calc(left, _, right) => {
            let left  = call(left, &input.jobs);
            let right = call(right, &input.jobs);

            match (left, right) {
                (Constant(n), Calc(f)) => f(n),
                (Calc(f), Constant(n)) => f(n),

                (Constant(_), Constant(_)) => panic!("didn't find a 'humn' node anywhere"),
                (Calc(_), Calc(_))         => panic!("found 'humn' node on both sides of tree")
            }
        },
        Job::Number(_) => panic!("expected the root node to be a calculation but it's a number")
    }
}
```


## Day 20 - Grove Positioning System

Today was all about permuting a cycle by shifting numbers around one at a time. After some time trying to mutably update the vector in place and not getting it right, I switched to a functional approach and wrote `shift_element()`. This function gluttonously *triples* the initial cycle before building a new one from a functional pipeline on it.  I was surprised it required three copies as I figured it should only require two, but it crashes on my input with only two. It probably only needs the first element of the third vector instead of the whole thing but I haven't tested the idea yet. I may be calculating the index too far to the right for certain inputs to the function, or misunderstanding something about the whole thing. But three copies of the vector does the trick.

This is the first time I put two trait bounds on the same generic type parameter, `T` in this case. It doesn't matter what the underlying vector element type is as long as the elements are `Clone`-able and equality-comparable with `PartialEq`:

```rust
// this is a functional way to move an element in a cycle to some other place in the cycle.
// it wastes a bit of space but it's conceptually easier to understand I think
fn shift_element<T>(vec    : &Vec<T>,
                    index  : usize,
                    offset : i64) -> Vec<T>
where T: Clone + PartialEq
{
    let len = vec.len() as i64;

    // line up three copies of the vector
    let tripled = [ vec.clone(), vec.clone(), vec.clone() ].concat();

    // wrap the offset to somewhere in our tripled vector
    let offset = (offset % (len-1) + len) % len
                 + if offset > 0 { 1 }
                   else          { 0 };

    [
        // put the indexed value at the front of the new vector
        vec![ vec[index].clone() ],

        // pull the rest of the cycle, making sure to filter out the one we put at the front
        tripled.into_iter()
               .skip(index + offset as usize)
               .filter(|el| *el != vec[index])
               .take(vec.len() - 1)
               .collect()
    ].concat()
}
```

During testing, I was surprised to find that the identity shift is obtained when the `offset` argument is set to *one less* than the cycle length instead of the length itself. Surely this is standard hooey for an abstract algebraist, but for me, it was a neat discovery about mathematics in the midst of a programming challenge.

```rust
// first moves len()-1.  note this is the identity shift, not len()!
assert_eq!(shift_element(&vec![1, 2, 3, 4, 5, 6, 7], 0, 7-1),
                          vec![1, 2, 3, 4, 5, 6, 7]);

// first moves len()
assert_eq!(shift_element(&vec![1, 2, 3, 4, 5, 6, 7], 0, 7),
                          vec![1, 3, 4, 5, 6, 7, 2]);
```

## Day 19 - Not Enough Minerals

This is the second day that I only got part 1 for. I couldn't figure out how to finish part 2 in a reasonable amount of time. As always, the problem seems to be carefully sized to get this effect for the second part unless we can think of some clever way to shortcut the computation, or find some pattern or something. But today it escaped me, and I still am not sure where to find the efficiency.


## Day 18 - Boiling Boulders

This one was really neat. We have to count exposed faces of a set of 3D cubes. In part one it's any exposed face, so any cube that doesn't jut up directly against another cube, we can count all six faces. In part two we discover the clump of cubes has an interior that lava can't get to, and this time we only want to count the faces the lava *can* get to, which amounts to a flood fill around the exterior, counting each face we bump into. The only clever thing I did today was bump the whole mess up/right/back 1 unit to ensure I can start the flood-fill at (0,0,0) and guarantee there's no cube there, and so I can reach around all sides of the cube during the fill.


## Day 17 - Pyroclastic Flow

This is the first day I only got the first part for. It's a tetris-like rock-falling simulation where the tiles don't rotate but they can be blown side-to-side by the jet-flow pattern.  We need to infinitely repeat two different sequences today, falling shapes and jet flow patterns, so I implemented a new iterator, `Repeater` that does the obvious thing by cloning the elements of the underlying iterator on the first pass, then repeating the cloning of the stored elements forever in a cycle on subsequent passes. It could probably be more efficient by only doing the clones the first time and always emitting references, but I didn't get around to it because...

Part 2 asks us to iterate the main rock-fall simulation a trillion times. With a lot of annoying text analysis of the output chamber output after a few thousand rounds, I was able to find a repeating pattern. After every 1705 stopped rocks (for my input) there are another 2618 occupied rows in the chamber, so from there it's a relatively easy job to determine how many rows there are after a trillion stopped rocks.  However by this point I was so put off by the grating nature of the whole task that I ended up throwing a small fit and decided to leave the second half for some other time.


## Day 16 - Proboscidea Volcanium

In this puzzle we have to find the most efficient way to spend 30 minutes traveling through a tunnel system with valves that can be turned on and which yield various amounts of pressure. The goal is to maximize the total pressure released in the 30 minutes by choosing the smartest route through the tunnels. The closest valve isn't necessarily the next most vital one to turn on to achieve this goal!  I initially considered how to cut the search tree down and thought of various greedy algorithms, such as go for the highest pressure valve first regardless of how far away it was, or go for the next nearest valve regardless of how much pressure it contributed. But I skipped these and hoped that a brute-force of the entire tree wouldn't take too long, reasoning that since we only have 30 minutes we don't have time to check all 15! leaves--most of them will never actually be visited before the algorithm uses up its 30 time steps.

Part two looked hopelessly complicated at first but turned out to be easier than I thought. With an elephant's movements to also worry about, I thought it would be a nightmare to track both us and the elephant at the same time, having to care that one node was reached before another one, etc. But I realized the two walks can be considered independent and independently. Look at it from the point of view of the end of any attempt to turn on the valves. It looks like we took one set of valves and the elephant took the complement of that set. So we ask, how many ways can I visit, say, 4 valves, while the elephant takes the other 11? Then just run the existing `best_path` function for each complement set, sum the two results and find the maximum pressure that way. It works, although a bit slowly at 10.7s total for both parts.


## Day 15 - Beacon Exclusion Zone

My first Iterator day! Good thing I completely missed the much quicker way to compute the uncovered cell, as it let me finally start to dig into the more interesting parts of Rust, which for today was a custom iterator implementation using the [Iterator trait](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html). A few times in the past I've thought about this problem of merging intervals but I always found a way around it, solving the problem in a different way. Today I got serious about it and wrote an iterator that wraps an underlying iterator of Intervals (integer ranges including both endpoints, which was already in the standard library as `std::ops::RangeInclusive<i32>`) and merges a new one into it at the appropriate place, keeping everything sorted and non-overlapping.

This wasn't my first approach to it though. I initially [asked ChatGPT what it would do](https://sharegpt.com/c/8QCfBy2) but even without testing it I can tell it's going to keep pushing the same new interval to the new list over and over again. I should write a few unit tests to confirm this but I'm not going to spend any more time on ChatGPT's ideas for today.

I briefly tried implementing my own in-place vector update but my Vec-fu wasn't up to the challenge.  It turns out github user `ephemient` came up with the code I was looking for, which is published here: https://github.com/ephemient/aoc2022/blob/main/rs/src/day15.rs#L13

The nice thing about the Iterator approach is that the only way to call the underyling iterator is through its `next()` method, so I'm forced to think about having only one Interval at a time instead of all of them at once, as is needed for `ephemient's` vector-update solution above. I found that having to consider only one interval at a time forced a more organized thought process, and with enough grit and unit testing I came up with [IntervalMerger Iterator](https://github.com/jasonincanada/aoc-2022/blob/main/days/day_15/src/main.rs#L120). It doesn't yield the most efficient solution but I'm proud of finally getting around to implementing a built-in Rust trait and having a working interval merger.

There's a lot to optimize for this day but I ran out of time and steam for it.

Here's the mutable state struct for `IntervalMerger`, omitting the actual `next()` call itself:

```rust
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
```


## Day 14 - Regolith Reservoir

Every year seems to have a falling-sand type thing, and this year's was day 14.  I did the obvious thing: simulate one grain of sand falling at a time, locking them into place when they had nowhere else to fall.


## Day 13 - Distress Signal

Finally a parsing task requiring more than the simple string chopping I've been using so far.  I resisted switching over to Haskell and its beautiful parsing combinators and instead wrote the Rust equivalent after learning to use its popular [nom crate](https://docs.rs/nom/latest/nom/). The most complicated it got was `parse_list` which recursively calls itself or parses a number and expects it all wrapped in a pair of `[`/`]`:

```rust
// [1,2,[3,4],[],5]
fn parse_list(s: &str) -> IResult<&str, Self> {
    let parser =
        delimited(
            tag("["),
            separated_list0(tag(","), alt((Packet::parse_number,
                                           Packet::parse_list))),
            tag("]")
        );

    map(parser, |list| { List(list) })
       (s)
}

// 10
fn parse_number(s: &str) -> IResult<&str, Self> {
    map(digit1, |num: &str| { Number(num.parse().unwrap())})
       (s)
}
```


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
