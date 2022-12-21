/*  https://adventofcode.com/2022/day/21  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { jobs : HashMap<Monkey, Job> }

type Monkey = String;

enum Job {
    Number(i64),
    Calc(String, char, String)
}

// simple recursive descent into an expression tree, evaluating the top node
fn part1(input: &Input) -> i64 {
    yell("root", &input.jobs)
}

fn yell(monkey: &str,
        jobs  : &HashMap<Monkey, Job>) -> i64
{
    match jobs.get(monkey).unwrap() {
        // yell out the number
        Job::Number(n) => *n,

        // recursively compute the number
        Job::Calc(left, operation, right) => {
            let left  = yell(left, jobs);
            let right = yell(right, jobs);

            match operation {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                 _  => panic!("unknown operation")
            }
        }
    }
}

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

fn call(monkey: &str,
        jobs  : &HashMap<Monkey, Job>) -> ResultOfDescent
{
    // start off our big composed function with the identity function
    if monkey == "humn" {
        return Calc(Box::new(|x| x))
    }

    match jobs.get(monkey).unwrap() {
        Job::Number(n) => Constant(*n),

        Job::Calc(left, operation, right) => {
            let left  = call(left, jobs);
            let right = call(right, jobs);

            // combine the left/right nodes in the 4 possible ways
            match (left, right) {
                (Constant(left), Constant(right)) => {
                    match operation {
                        '+' => Constant(left + right),
                        '-' => Constant(left - right),
                        '*' => Constant(left * right),
                        '/' => Constant(left / right),
                         _  => panic!("unknown operation")
                    }
                },
                (Constant(n), Calc(f)) => {
                    match operation {
                        '+' => Calc(Box::new(move |x| f(x - n))), // assert x >= n?
                        '-' => Calc(Box::new(move |x| f(n - x))),
                        '*' => Calc(Box::new(move |x| f(x / n))),
                        '/' => Calc(Box::new(move |x| f(n / x))),
                         _  => panic!("unknown operation")
                    }
                },
                (Calc(f), Constant(n)) => {
                    match operation {
                        '+' => Calc(Box::new(move |x| f(x - n))),
                        '-' => Calc(Box::new(move |x| f(n + x))),
                        '*' => Calc(Box::new(move |x| f(x / n))),
                        '/' => Calc(Box::new(move |x| f(n * x))),
                         _  => panic!("unknown operation")
                    }
                },
                (Calc(_), Calc(_)) =>
                    panic!("should never get here if there's only one 'humn' node in the tree"),
            }
        }
    }
}

// the result of evaluating a sub-expression is either a constant or a function
// that takes a number requested from above and calculates the missing operand
enum ResultOfDescent {
    Constant(i64),
    Calc(Box<dyn Fn(i64) -> i64>)
}

use ResultOfDescent::*;


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            jobs: s.lines()
                   .map(Input::from_line)
                   .into_iter()
                   .collect()
        }
    }

    // "root: pppw + sjmn"
    fn from_line(line: &str) -> (String, Job) {
        let (monkey, job) = line.split_once(": ").unwrap();
        (monkey.to_string(), Job::from(job))
    }
}

impl Job {
    // "10"
    // "pppw + sjmn"
    fn from(s: &str) -> Self {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        if tokens.len() == 1 {
            Job::Number(tokens[0].parse().unwrap())
        } else {
            Job::Calc(tokens[0].to_string(),
                      tokens[1].chars().next().unwrap(),
                      tokens[2].to_string())
        }
    }
}


/* Imports */

use std::collections::HashMap;


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call() {

        // simplest case, it will ignore the 99 and return 5
        assert_eq!(part2(&Input::from_string("root: humn + five\n\
                                              humn: 99\n\
                                              five: 5")),
                         5);

        // n + f
        assert_eq!(part2(&Input::from_string("root: fifty . add\n\
                                              add: thirtyfive + plus\n\
                                              thirtyfive: 35\n\
                                              plus: humn + five\n\
                                              five: 5\n\
                                              fifty: 50")),
                         10);

        // n - f
        assert_eq!(part2(&Input::from_string("root: five . sub\n\
                                              sub: thirtyfive - plus\n\
                                              thirtyfive: 35\n\
                                              plus: humn + five\n\
                                              five: 5")),
                         25);

        // n * f
        assert_eq!(part2(&Input::from_string("root: threefifty . mult\n\
                                              mult: thirtyfive * plus\n\
                                              thirtyfive: 35\n\
                                              plus: humn + five\n\
                                              threefifty: 350\n\
                                              five: 5")),
                         350/35-5);

        // n / f
        assert_eq!(part2(&Input::from_string("root: five . div\n\
                                              div: thirtyfive / plus\n\
                                              thirtyfive: 35\n\
                                              plus: humn + five\n\
                                              five: 5")),
                         2);

        // f + n
        assert_eq!(part2(&Input::from_string("root: fifty . add\n\
                                              add: plus + thirtyfive\n\
                                              thirtyfive: 35\n\
                                              plus: humn + five\n\
                                              five: 5\n\
                                              fifty: 50")),
                         10);

        // f - n
        assert_eq!(part2(&Input::from_string("root: five . sub\n\
                                              sub: plus - thirtyfive\n\
                                              thirtyfive: 35\n\
                                              plus: humn + five\n\
                                              five: 5")),
                         35);

        // f * n
        assert_eq!(part2(&Input::from_string("root: threefifty . mult\n\
                                              mult: plus * thirtyfive\n\
                                              thirtyfive: 35\n\
                                              plus: humn + five\n\
                                              threefifty: 350\n\
                                              five: 5")),
                         350/35-5);

        // f / n
        assert_eq!(part2(&Input::from_string("root: five . div\n\
                                              div: plus / thirtyfive\n\
                                              thirtyfive: 35\n\
                                              plus: humn + five\n\
                                              five: 5")),
                         5*35-5);
    }

    #[test] fn test_part1() { assert_eq!(part1(&get_example()), 152); }
    #[test] fn test_part2() { assert_eq!(part2(&get_example()), 301); }

    #[test] fn test_part1_mine() { assert_eq!(part1(&Input::from("input.txt")), 142_707_821_472_432); }
    #[test] fn test_part2_mine() { assert_eq!(part2(&Input::from("input.txt")), 3_587_647_562_851); }

    fn get_example() -> Input {
        Input::from_string(
            "root: pppw + sjmn\n\
             dbpl: 5\n\
             cczh: sllz + lgvd\n\
             zczc: 2\n\
             ptdq: humn - dvpt\n\
             dvpt: 3\n\
             lfqf: 4\n\
             humn: 5\n\
             ljgn: 2\n\
             sjmn: drzm * dbpl\n\
             sllz: 4\n\
             pppw: cczh / lfqf\n\
             lgvd: ljgn * ptdq\n\
             drzm: hmdt - zczc\n\
             hmdt: 32"
        )
    }
}

/*  $ time target/release/day_21.exe
    Part 1: 142707821472432
    Part 2: 3587647562851

    real    0m0.019s
*/
