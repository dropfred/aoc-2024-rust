#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
    Concat
}

struct Equation {
    result: u64,
    numbers: Vec<u64>
}

impl Equation {
    fn is_valid<const NOPS: usize>(&self, ops: &[Op; NOPS]) -> bool {
        assert_ne!(NOPS, 0);
        if self.numbers.is_empty() {
            return false;
        }
        let apply = |op: Op, a: u64, b: u64| {
            match op {
                Op::Add => a + b,
                Op::Mul => a * b,
                Op::Concat => (a * 10u64.pow(b.ilog10() + 1)) + b
            }
        };
        let mut ns: Vec<(u64, usize)> = Vec::with_capacity(self.numbers.len());
        for (i, n) in self.numbers.iter().enumerate() {
            if i > 0 {
                ns.push((apply(ops[0], ns[i - 1].0, *n), 0));
            } else {
                ns.push((*n, 0));
            }
        }
        while ns.last().unwrap().0 != self.result {
            let mut i = ns.len() - 1;
            loop {
                if i == 0 {
                    return false;
                }
                ns[i].1 += 1;
                if ns[i].1 < NOPS {
                    break;
                }
                ns[i].1 = 0;
                i -= 1;
            }
            for i in i..ns.len() {
                ns[i].0 = apply(ops[ns[i].1], ns[i - 1].0, self.numbers[i]);
            }
        }
        true
    }
}

struct Puzzle {
    equations: Vec<Equation>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_equation = |s: &str| {
            let (result, numbers) = s.trim().split_once(": ")?;
            let result = result.parse().ok()?;
            let numbers: Option<Vec<_>> = numbers.split(' ').map(|v| v.parse().ok()).collect();
            let numbers = numbers?;
            Some(Equation {result, numbers})
        };
        let equations: Option<Vec<_>> = data.trim().lines().map(parse_equation).collect();
        let equations = equations?;
        Some(Puzzle {equations})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> u64 {
    puzzle.equations.iter()
        .filter(|e| e.is_valid(&[Op::Add, Op::Mul]))
        .map(|e| e.result)
        .sum()
}

fn part_2(puzzle: &Puzzle) -> u64 {
    puzzle.equations.iter()
        .filter(|e| e.is_valid(&[Op::Add, Op::Mul, Op::Concat]))
        .map(|e| e.result)
        .sum()
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_07/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

mod tests {
    use super::*;

    const DATA: &str = include_str!("../../data/day_07/test.txt");

    #[test]
    fn test_part_1() {
        let data = Puzzle::load(DATA);
        assert_eq!(part_1(&data), 3749);
    }

    #[test]
    fn test_part_2() {
        let data = Puzzle::load(DATA);
        assert_eq!(part_2(&data), 11387);
    }
}
