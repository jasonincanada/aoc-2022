/*  https://adventofcode.com/2022/day/11  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { monkeys: Vec<Monkey> }

struct Monkey {
    items        : Vec<u64>,
    operation    : Operation,
    divisible_by : u64,
    if_true      : usize,
    if_false     : usize
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Multiply(u64),
    Add(u64),
    Square
}

// 20 rounds and the custom operation to keep from overflowing is to divide by 3
fn part1(input: &Input) -> usize {    
    monkey_in_the_middle(&input.monkeys,
                         20,
                         |worry| worry / 3)
}

// 10,000 rounds and the custom operation is to mod by the product of the divisors
fn part2(input: &Input) -> usize {
    
    let product_of_divisors =
        input.monkeys.iter()
                     .map(|m| m.divisible_by as u32)
                     .product::<u32>() as u64;

    monkey_in_the_middle(&input.monkeys,
                         10_000,
                         |worry| worry % product_of_divisors)
}

fn monkey_in_the_middle<F>(monkeys: &Vec<Monkey>,
                           rounds: usize,
                           custom_op: F) -> usize
    where F: Fn(u64) -> u64
{
    // track the number of inspections by each monkey
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];

    // clone the items queues because we'll be mutating them in place
    let mut items: Vec<Vec<u64>> = monkeys.iter()
                                          .map(|monkey| monkey.items.clone())
                                          .collect();

    for _ in 0..rounds {
        for (m, monkey) in monkeys.iter().enumerate() {

            // drain this monkey's items into their own vector so we can iterate over them,
            // otherwise rust complains about two references to the items vec at the same time
            let worries: Vec<u64> = items[m].drain(..).collect();

            // do an operation on each worry level in the monkey's list
            for worry in worries {
                let worry = operate(&monkey.operation, worry);
                let worry = custom_op(worry);

                let catcher = if worry % monkey.divisible_by == 0 {
                                  monkey.if_true
                              } else {
                                  monkey.if_false
                              };

                items[catcher].push(worry);

                // tally an inspection for this monkey
                inspections[m] += 1;
            }
        }
    }

    // find the two most active monkeys
    inspections.sort();
    inspections.into_iter()
               .rev()
               .take(2)
               .product()
}

fn operate(operation: &Operation, worry: u64) -> u64 {
    match operation {
        Multiply(x) => worry * x,
        Add(x)      => worry + x,
        Square      => worry * worry
    }
}


/* Parsing */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        Input {
            monkeys: s.split("\n\n")
                      .map(Monkey::from_string)
                      .collect()
        }
    }
}

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

impl Operation {
    fn from_expression(expr: &str) -> Self {
        // the left half of the expression is always "old"
        let rest   = expr.strip_prefix("old ").unwrap();
        let tokens = rest.split_whitespace().collect::<Vec<&str>>();

        match tokens[0] {
            "+" => Add(tokens[1].parse().unwrap()),
            "*" => match tokens[1] {
                      "old" => Square,
                       num  => Multiply(num.parse().unwrap())
                   },
             _  => panic!("Unknown operation")
        }
    }
}

use Operation::*;


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_example()), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_example()), 2713310158);
    }

    #[test]
    fn test_parse_monkey() {
        let monkey = Monkey::from_string("Monkey 0:\n\
                                            Starting items: 79, 98\n\
                                            Operation: new = old * 19\n\
                                            Test: divisible by 23\n\
                                              If true: throw to monkey 2\n\
                                              If false: throw to monkey 3");

        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!(monkey.operation, Multiply(19));
        assert_eq!(monkey.divisible_by, 23);
        assert_eq!(monkey.if_true, 2);
        assert_eq!(monkey.if_false, 3);
    }

    #[test]
    fn test_operation_from_expression() {
        assert_eq!(Operation::from_expression("old * 12" ), Multiply(12));
        assert_eq!(Operation::from_expression("old + 12" ), Add(12));
        assert_eq!(Operation::from_expression("old * old"), Square);
    }

    fn get_example() -> Input {
        Input::from("example.txt")
    }
}

/*  $ grep Test input.txt | sort
    Test: divisible by 11
    Test: divisible by 13
    Test: divisible by 17
    Test: divisible by 19
    Test: divisible by 2
    Test: divisible by 3
    Test: divisible by 5
    Test: divisible by 7

    λ> product [2,3,5,7,11,13,17,19]
    9699690

    λ> logBase 2 (product [2,3,5,7,11,13,17,19])
    23.209507209138437

    λ> logBase 2 (square $ product [2,3,5,7,11,13,17,19])
    46.41901441827687

    $ cargo run   
    Part 1: 50830
    Part 2: 14399640002
 */
