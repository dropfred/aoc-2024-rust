use std::collections::{HashMap, HashSet};

struct Puzzle {
    secrets: Vec<u64>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let secrets: Result<_, _> = data.trim().lines().map(|s| s.trim().parse()).collect();
        let secrets = secrets.ok()?;
        Some(Puzzle {secrets})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn get_next_secret(secret: u64) -> u64 {
    let mut s = secret;
    s = ((s * 64  ) ^ s) % 16777216;
    s = ((s / 32  ) ^ s) % 16777216;
    s = ((s * 2048) ^ s) % 16777216;
    s
}

fn get_nth_secret(secret: u64, nth: usize) -> u64 {
    let mut s = secret;
    for _ in 0..nth {
        s = get_next_secret(s);
    }
    s
}

fn part_1(puzzle: &Puzzle) -> u64 {
    puzzle.secrets.iter().map(|s| get_nth_secret(*s, 2000)).sum()
}

fn part_2_n(puzzle: &Puzzle, n: usize) -> (u64, (i8, i8,i8, i8)) {
    let mut ps = HashMap::new();
    for s in &puzzle.secrets {
        let mut ss = Vec::new();
        let mut s = *s;
        for _ in 0..n {
            let p = (s % 10) as u8;
            let ns = get_next_secret(s);
            let np = (ns % 10) as u8;
            let dp = (np as i8) - (p as i8);
            ss.push((np, dp));
            s = ns;
        }
        let mut nps = HashSet::new();
        for ss in ss.windows(4) {
            let p = ss[3].0;
            let ss = (ss[0].1, ss[1].1, ss[2].1, ss[3].1);
            if !nps.contains(&ss) {
                nps.insert(ss);
                let mp = ps.entry(ss).or_insert(0);
                *mp += p as u64;
            }
        }
    }
    let (p, ss)  = ps.into_iter().map(|(ss, p)| (p, ss)).max().unwrap();
    (p, ss)
}

fn part_2(puzzle: &Puzzle) -> u64 {
    part_2_n(puzzle, 2000).0
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_22/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1:  {}", part_1(&puzzle));
    println!("part 2:  {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_secret() {
        let mut s = 123;
        s = get_next_secret(s); assert_eq!(s, 15887950);
        s = get_next_secret(s); assert_eq!(s, 16495136);
        s = get_next_secret(s); assert_eq!(s, 527345);
        s = get_next_secret(s); assert_eq!(s, 704524);
        s = get_next_secret(s); assert_eq!(s, 1553684);
        s = get_next_secret(s); assert_eq!(s, 12683156);
        s = get_next_secret(s); assert_eq!(s, 11100544);
        s = get_next_secret(s); assert_eq!(s, 12249484);
        s = get_next_secret(s); assert_eq!(s, 7753432);
        s = get_next_secret(s); assert_eq!(s, 5908254);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_22/test_1.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 37327623);
    }

    #[test]
    fn test_part_2_n() {
        let data = "123";
        let puzzle = Puzzle::load(data);
        let (p, ss) = part_2_n(&puzzle, 9);
        assert_eq!(p, 6);
        assert_eq!(ss, (-1, -1, 0, 2));
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_22/test_2.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_2(&puzzle), 23);
    }
}
