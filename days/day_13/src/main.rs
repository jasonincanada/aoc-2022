/*  https://adventofcode.com/2022/day/13  */

fn main() {
    let input = Input::from("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Input { pairs: Vec<(Packet, Packet)> }

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Number(u8),
    List(Vec<Packet>)
}

// sum the indices of the pairs that are in the right order
fn part1(input: &Input) -> usize {
    input.pairs.iter()
               .map(|(left, right)| in_right_order(left, right))
               .enumerate()
               .map(|(i, ordering)| if ordering == Ordering::Less {i+1} else {0})
               .sum()
}

// find the indices of the two new packets in the ordered list of all the packets
fn part2(input: &Input) -> usize {

    // combine all the pairs into a list
    let mut pairs: Vec<Packet> =
        input.pairs.iter()
                   .flat_map(|(left, right)| vec![left.clone(), right.clone()])
                   .collect();

    // add the two new packets defined in the problem description
    let new1 = Packet::from_string("[[2]]");
    let new2 = Packet::from_string("[[6]]");

    pairs.push(new1.clone());
    pairs.push(new2.clone());

    pairs.sort_by(in_right_order);

    let index1 = pairs.iter().position(|val| val.clone() == (&new1).clone()).unwrap() + 1;
    let index2 = pairs.iter().position(|val| val.clone() == (&new2).clone()).unwrap() + 1;

    index1 * index2
}

fn in_right_order(left : &Packet,
                  right: &Packet) -> Ordering
{
    match (left, right) {
        (Number(l), Number(r)) => l.cmp(r),

        // if we're comparing a number to a list
        (Number(num), right) => in_right_order(&Packet::list_from_number(*num), right),
        (left, Number(num) ) => in_right_order(left, &Packet::list_from_number(*num)),

        (List(left), List(right)) => {
            if left.is_empty() && right.is_empty() { return Ordering::Equal   }
            if left.is_empty()                     { return Ordering::Less    }
            if right.is_empty()                    { return Ordering::Greater }

            match in_right_order(&left[0], &right[0]) {
                Ordering::Equal => {
                    let l: Vec<Packet> = left. iter().skip(1).cloned().collect();
                    let r: Vec<Packet> = right.iter().skip(1).cloned().collect();

                    in_right_order(&List(l), &List(r))
                }

                ordering => ordering
            }
        }
    }
}

/* Parsing  */

impl Input {
    fn from(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).expect("Couldn't read input");
        Input::from_string(contents.trim())
    }

    fn from_string(s: &str) -> Self {
        let pairs = s.split("\n\n");

        Input {
            pairs: pairs.map(|pair| { let (left, right) = pair.split_once('\n').unwrap();

                                      (Packet::from_string(left),
                                       Packet::from_string(right))
                                    })
                        .collect()
        }
    }
}

impl Packet {
    // 10
    fn parse_number(s: &str) -> IResult<&str, Self> {
        map(digit1, |num: &str| { Number(num.parse().unwrap())})
           (s)
    }

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

    fn list_from_number(num: u8) -> Packet {
        List(vec![Number(num)])
    }

    fn from_string(s: &str) -> Self {
        Packet::parse_list(s).unwrap().1
    }
}

/* Imports */

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use std::cmp::Ordering;
use Packet::*;

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_example()), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_example()), 140);
    }

    #[test]
    fn parse_number() {
        let x = Packet::parse_number("23u");
        assert_eq!(x.unwrap(), ("u", Packet::Number(23)));
    }

    #[test]
    fn parse_list() {
        let list = Packet::parse_list("[[1,2],3,4,[]]").unwrap();
        let expected = List(vec![List(vec![Number(1),
                                           Number(2)]),
                                 Number(3),
                                 Number(4),
                                 List(vec![])]);

        assert_eq!(list, ("", expected));
    }


    fn get_example() -> Input {
        Input::from("example.txt")
    }
}
