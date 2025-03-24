use std::collections::HashMap;

struct Puzzle {
    locations: Vec<(u32, u32)>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_locations = |s: &str| {
            let mut vs = s.split_ascii_whitespace();
            let v0 = vs.next()?.parse().ok()?;
            let v1 = vs.next()?.parse().ok()?;
            if vs.next().is_some() {return None;}
            Some((v0, v1))
        };
        let locations: Option<_> = data.trim().lines().map(parse_locations).collect();
        let locations = locations?;
        Some(Puzzle {locations})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> u32 {
    let (mut v0s, mut v1s): (Vec<_>, Vec<_>) = puzzle.locations.iter().cloned().unzip();
    v0s.sort();
    v1s.sort();
    v0s.into_iter().zip(v1s.into_iter()).map(|(v0, v1)| if v0 > v1 {v0 - v1} else {v1 - v0}).sum()
}

fn part_2(puzzle: &Puzzle) -> u32 {
    let mut v2 : HashMap<u32, u32> = HashMap::new();
    for (_, v1) in &puzzle.locations {
        *v2.entry(*v1).or_insert(0) += 1;
    }
    let mut distance = 0;
    for (v0, _) in &puzzle.locations {
        if v2.contains_key(v0) {
            distance += v2[&v0] * v0;
        }
    }
    distance
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_01/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_01/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 11);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_01/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_2(&puzzle), 31);
    }
}
