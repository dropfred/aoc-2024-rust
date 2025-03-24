struct Puzzle;

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        None
    }
 
    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn solve_part_1(puzzle: &Puzzle) -> u32 {
    todo!("part 1")
}

fn solve_part_2(puzzle: &Puzzle) -> u32 {
    todo!("part 2")
}

pub fn solve() {
    let puzzle = include_str!("../../data/day_xx/input.txt");
    let puzzle = Puzzle::load(puzzle);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let puzzle = include_str!("../../data/day_xx/test.txt");
        let puzzle = parse_puzzle(puzzle);
    }

    #[test]
    fn test_part_1() {
        let puzzle = include_str!("../../data/day_xx/test.txt");
        let puzzle = parse_puzzle(puzzle);
        assert!(part_1(&puzzle) == 0);
    }

    #[test]
    fn test_part_2() {
        let puzzle = include_str!("../../data/day_xx/test.txt");
        let puzzle = parse_puzzle(puzzle);
        assert!(part_2(&puzzle) == 0);
    }
}
