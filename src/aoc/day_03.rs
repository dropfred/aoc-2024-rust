struct Puzzle {
    memory: String
}

impl Puzzle {
    fn load(data: &str) -> Puzzle {
        assert!(data.is_ascii());
        Self {memory: data.to_string()}
    }
}

fn part_1(puzzle: &Puzzle) -> u64 {
    let mut total = 0;
    let memory = &puzzle.memory;
    for (i, _) in memory.match_indices("mul(") {
        let (_, s) = memory.split_at(i + 4);
        if let Some(e) = s.find(')') {
            let s = &s[..e];
            if s.len() > 7 {continue;}
            let mut ns = s.split(|c| c as char == ',');
            let a = ns.next();
            let b = ns.next();
            if a.is_none() || b.is_none() {continue;}
            if ns.next().is_some() {continue;}
            let a = a.unwrap();
            let b = b.unwrap();
            let a = u64::from_str_radix(a, 10);
            let b = u64::from_str_radix(b, 10);
            if a.is_err() || b.is_err() {continue;}
            let a = a.unwrap();
            let b = b.unwrap();
            total += a * b;
        }
    }
    total
} 

fn part_2(puzzle: &Puzzle) -> u64 {
    const DO: &str = "do()";
    const DONT: &str = "don't()";
    const MUL: &str = "mul(";
    let m = &puzzle.memory;
    let mut b = 0;
    let mut enable = true;
    let mut total = 0;
    while b < m.len() {
        let s = &m[b..];
        if s.starts_with(DO) {
            b += DO.len();
            enable = true;
        } else if s.starts_with(DONT) {
            b += DONT.len();
            enable = false;
        } else if s.starts_with(MUL) {
            b += MUL.len();
            let s = &s[MUL.len()..];
            if let Some(e) = s.find(')') {
                let s = &s[..e];
                if s.len() > 7 {continue;}
                let mut ns = s.split(|c| c as char == ',');
                let n1 = ns.next();
                let n2 = ns.next();
                if n1.is_none() || n2.is_none() {continue;}
                if ns.next().is_some() {continue;}
                let n1 = n1.unwrap();
                let n2 = n2.unwrap();
                let n1 = u64::from_str_radix(n1, 10);
                let n2 = u64::from_str_radix(n2, 10);
                if n1.is_err() || n2.is_err() {continue;}
                let n1 = n1.unwrap();
                let n2 = n2.unwrap();
                if enable {
                    total += n1 * n2;
                }
                b += e + 1;
            }
        } else {
            b += 1;
        }
    }
    total
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_03/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 161);
    }

    #[test]
    fn test_part_2() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let puzzle = Puzzle::load(data);
        assert_eq!(part_2(&puzzle), 48);
    }
}