use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Data {
    patterns: Vec<String>,
    designs: Vec<String>
}

impl Data {
    fn parse(data: &str) -> Option<Self> {
        let mut data = data.trim().lines();
        let patterns = data.next()?.split(", ").map(String::from).collect();
        data.next();
        let designs = data.map(String::from).collect();
        Some(Data {patterns, designs})
    }
}

fn count(design: &str, patterns: &Vec<String>) -> usize {
    fn count(memo: &mut HashMap<usize, usize>, design: &str, patterns: &Vec<String>, s: usize) -> usize {
        if design.len() == s {
            1
        } else if memo.contains_key(&s) {
            memo[&s]
        } else {
            let d = &design[s..];
            patterns.iter().filter(|p| d.starts_with(*p)).map(|p| {
                let s = s + p.len();
                let c = count(memo, design, patterns, s);
                memo.insert(s, c);
                c
            }).sum()
        }
    }
    count(&mut HashMap::new(), design, patterns, 0)
}

fn part_1(data: &Data) -> usize {
    data.designs.iter().map(|d| count(d, &data.patterns)).filter(|n| *n > 0).count()
}

fn part_2(data: &Data) -> usize {
    data.designs.iter().map(|d| count(d, &data.patterns)).sum()
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_19/input.txt");
    let data = Data::parse(data).expect("bad input");

    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_19/test.txt");
        let data = Data::parse(data).unwrap();
        assert_eq!(data.patterns.len(), 8);
        assert_eq!(data.designs.len(), 8);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_19/test.txt");
        let data = Data::parse(data).unwrap();
        assert_eq!(part_1(&data), 6);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_19/test.txt");
        let data = Data::parse(data).unwrap();
        assert_eq!(part_2(&data), 16);
    }
}
