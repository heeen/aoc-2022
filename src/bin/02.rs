use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            match (self, other) {
                (Shape::Rock, Shape::Scissor)
                | (Shape::Paper, Shape::Rock)
                | (Shape::Scissor, Shape::Paper) => Some(Ordering::Greater),
                (_, _) => Some(Ordering::Less),
            }
        }
    }
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

#[derive(Debug)]
struct Round {
    mine: Shape,
    response: Shape,
}

impl Round {
    fn score(&self) -> u32 {
        let points = if self.mine > self.response {
            6
        } else if self.mine == self.response {
            3
        } else {
            0
        };

        let result = points + self.mine.value();
        println!(
            "hand value {:?} {}  vs {:?} score: {} total {}",
            self.mine,
            self.mine.value(),
            self.response,
            points,
            result
        );
        result
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rounds = input
        .lines()
        .map(|l| l.as_bytes())
        .map(|t| Round {
            mine: match t[0] {
                b'A' => Shape::Rock,
                b'B' => Shape::Paper,
                b'C' => Shape::Scissor,
                _ => unreachable!(),
            },
            response: match t[2] {
                b'X' => Shape::Rock,
                b'Y' => Shape::Paper,
                b'Z' => Shape::Scissor,
                _ => unreachable!(),
            },
        });

    Some(rounds.map(|r| r.score()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!(Shape::Rock > Shape::Scissor);
        assert!(Shape::Scissor < Shape::Rock);

        assert!(Shape::Paper > Shape::Rock);
        assert!(Shape::Rock < Shape::Paper);

        assert!(Shape::Scissor > Shape::Paper);
        assert!(Shape::Paper < Shape::Scissor);

        assert!(Shape::Scissor == Shape::Scissor);
        assert!(Shape::Rock == Shape::Rock);
        assert!(Shape::Paper == Shape::Paper);
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
