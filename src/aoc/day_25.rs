use crate::aoc::grid::Grid;

struct Puzzle
{
    locks: Vec<[u8; 5]>,
    keys: Vec<[u8; 5]>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        #[derive(PartialEq)]
        enum LK {L, K}
        struct LKS {
            lk: LK,
            schema: [u8; 5]
        }
        let parse_lk = |d: &str| {
            let grid: Grid<char> = Grid::parse(d, "")?;
            if grid.size() != (5, 7) {return None;}
            if grid.get((0, 0)) == grid.get((0, 6)) {return None;}
            if (1..5).any(|x| grid.get((x, 0)) != grid.get((0, 0))) {return None;}
            if (1..5).any(|x| grid.get((x, 6)) != grid.get((0, 6))) {return None;}
            for y in 0..7 {
                for x in 0..5 {
                    if !"#.".contains(grid.get((x, y))) {
                        return None;
                    }
                }
            }
            let lks: LKS = if grid.get((0, 0)) == '#' {
                let mut s = [0u8; 5];
                for x in 0..5 {
                    for y in 1..6 {
                        if grid.get((x, y)) != '#' {break;}
                        s[x] += 1;
                    }
                    for y in (s[x]as usize + 1)..6 {
                        if grid.get((x, y)) != '.' {return None;}
                    }
                }
                LKS {lk: LK::L, schema: s}
            } else {
                let mut s = [0u8; 5];
                for x in 0..5 {
                    for y in 1..6 {
                        if grid.get((x, 6 - y)) != '#' {break;}
                        s[x] += 1;
                    }
                    for y in (s[x]as usize + 1)..6 {
                        if grid.get((x, 6 - y)) != '.' {return None;}
                    }
                }
                LKS {lk: LK::K, schema: s}
            };
            Some(lks)
        };
        let data = data.trim().replace("\r", "");
        let lkss: Option<Vec<_>> = data.split("\n\n").map(parse_lk).collect();
        let lkss = lkss?;
        let (locks, keys): (Vec<_>, Vec<_>) = lkss.iter().partition(|lks| lks.lk == LK::L);
        let mut locks: Vec<_> = locks.into_iter().map(|lks| lks.schema).collect();
        let mut keys: Vec<_> = keys.into_iter().map(|lks| lks.schema).collect();
        locks.sort();
        keys.sort();
        Some(Puzzle {locks, keys})
    }
 
    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn solve_part_1(puzzle: &Puzzle) -> usize {
    puzzle.locks.iter().map(|lock| {
        puzzle.keys.iter().filter(|key| {
            (0..5).all(|i| (lock[i] + key[i]) <= 5)
        }).count()
    }).sum()
}

fn solve_part_2(puzzle: &Puzzle) -> u32 {
    todo!("part 2")
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_25/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_25/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(solve_part_1(&puzzle), 3);
    }
}
