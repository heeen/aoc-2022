use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let elfs = parse_elfs(&input);
    elfs.iter().max_by_key(|e| e.sum).map(|e| e.sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elfs = parse_elfs(&input);
    Some(
        elfs.iter()
            .sorted_by_key(|e| e.sum)
            .rev()
            .take(3)
            .map(|e| e.sum)
            .sum(),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[derive(Debug)]
struct Elf {
    items: Vec<u32>,
    sum: u32,
}

fn parse_elfs(input: &str) -> Vec<Elf> {
    let lines = input.lines().collect_vec();
    let elfs = lines
        .split(|l| l.len() == 0)
        .map(|r| {
            let items: Vec<_> = r.into_iter().map(|s| s.parse::<u32>().unwrap()).collect();

            let sum = items.iter().sum();

            Elf { sum, items }
        })
        .collect_vec();
    elfs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
