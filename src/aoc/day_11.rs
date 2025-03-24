use std::collections::HashMap;

struct Puzzle {
    stones: Vec<u64>
}

impl Puzzle {
    fn parse(data: &str) -> Option <Self> {
        let stones: Option<Vec<_>> = data.trim().split(' ').map(|s| s.parse().ok()).collect();
        let stones = stones?;
        Some(Self {stones})
    }

    
    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }

    fn blink(&self, blinks: u8) -> u64 {
        let mut ss = self.stones.iter().fold(HashMap::new(), |mut m, v| {*m.entry(*v).or_insert(0) += 1; m});
        for _ in 0..blinks {
            let mut nss = HashMap::new();
            for (v, n) in ss {
                if v == 0 {
                    *nss.entry(1).or_insert(0) += n;
                } else {
                    let ss = v.ilog10() + 1;
                    if (ss & 1) == 0 {
                        let p = 10u64.pow(ss / 2);
                        *nss.entry(v / p).or_insert(0) += n;
                        *nss.entry(v % p).or_insert(0) += n;
                    } else {
                        *nss.entry(v * 2024).or_insert(0) += n;
                    }
                }
            }
            ss = nss;
        }
        ss.values().fold(0, |t, n| t + n)
    }
}

fn part_1(puzzle: &Puzzle) -> u64 {
    puzzle.blink(25)
}

fn part_2(puzzle: &Puzzle) -> u64 {
    puzzle.blink(75)
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_11/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_11/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(puzzle.stones.len(), 2);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_11/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 55312);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_11/input.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_2(&puzzle), 221632504974231);
    }
}
