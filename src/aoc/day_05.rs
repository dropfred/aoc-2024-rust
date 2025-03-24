use std::{cmp::Ordering, collections::HashMap};

struct Puzzle {
    rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>
}

impl Puzzle {
    fn parse(data: &str) -> Option <Self> {
        let data: Vec<_> = data.trim().lines().map(|s| s.trim()).collect();
        let (order, updates) = {
            let s = data.iter().position(|s| s.is_empty())?;
            (data[..s].iter(), data[(s + 1)..].iter())
        };

        let mut rules = HashMap::new();
        for update in order {
            let (p1, p2) = update.split_once('|')?;
            let (p1, p2) = (p1.parse().ok()?, p2.parse().ok()?);
            rules.entry(p1).or_insert(Vec::new()).push(p2);
        }

        let parse_update = |ps: &&str| {
            let ps: Option<Vec<_>> = ps.split(',').map(|p| p.parse().ok()).collect();
            Some(ps?)
        };
        let updates: Option<Vec<_>> = updates.map(parse_update).collect();
        let updates = updates?;

        Some(Puzzle {rules, updates})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn is_valid_update(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    update.iter().enumerate().all(|(i, p)| {
        !rules.get(p).is_some_and(|rs| {rs.iter().any(|p| update[..i].contains(p))})
    })
}

fn part_1(data: &Puzzle) -> u32 {
    data.updates.iter()
        .filter(|u| is_valid_update(u, &data.rules))
        .map(|u| u[u.len() / 2])
        .sum()
}


fn part_2(data: &Puzzle) -> u32 {
    data.updates.iter()
        .filter(|u| !is_valid_update(u, &data.rules))
        .map(|u| {
            let mut u = u.clone();
            u.sort_by(|p1, p2| {
                if      data.rules.get(p1).is_some_and(|ps| {ps.contains(p2)}) {Ordering::Less}
                else if data.rules.get(p2).is_some_and(|ps| {ps.contains(p1)}) {Ordering::Greater}
                else {Ordering::Equal}
            });
            u[u.len() / 2]
        })
        .sum()
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_05/input.txt"); // 6951 / 4121
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_05/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(puzzle.updates.len(), 6);
        assert!(puzzle.updates.iter().all(|u| (u.len() & 1) != 0));
        assert_eq!(puzzle.rules.len(), 6);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_05/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 143);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_05/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_2(&puzzle), 123);
    }
}
