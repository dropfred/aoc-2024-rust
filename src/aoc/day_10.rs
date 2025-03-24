use std::collections::HashSet;
use crate::aoc::grid::Grid;

struct Puzzle {
    map:Grid<u8>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let map= Grid::parse(data, "")?;
        Some(Self {map})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }

    fn score(&self, x: usize, y: usize) -> u32 {
        if self.map.get((x, y)) != 0 {return 0;}
        let (w, h) = self.map.size();
        let (w, h) = (w as i32, h as i32);
        let mut ps = HashSet::new();
        ps.insert((x as i32, y as i32));
        for z in 1..=9 {
            let mut nps = HashSet::new();
            for (x, y) in &ps {
                let (x, y) = (*x, *y);
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if (nx >= 0) && (nx < w as i32) && (ny >= 0) && (ny < h as i32) && (self.map.get((nx as usize, ny as usize)) == z) {
                        nps.insert((nx, ny));
                    }
                }
            }
            ps = nps;
        }
        ps.len() as u32
    }

    fn rating(&self, x: usize, y: usize) -> u32 {
        if self.map.get((x, y)) != 9 {return 0;}
        let (w, h) = self.map.size();
        let (w, h) = (w as i32, h as i32);
        let mut ps = Vec::new();
        ps.push((x as i32, y as i32));
        for z in (0..=8).rev() {
            let mut nps = Vec::new();
            for (x, y) in &ps {
                let (x, y) = (*x, *y);
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if (nx >= 0) && (nx < w as i32) && (ny >= 0) && (ny < h as i32) && (self.map.get((nx as usize, ny as usize)) == z) {
                        nps.push((nx, ny));
                    }
                }
            }
            ps = nps;
        }
        ps.len() as u32
    }
}

fn part_1(puzzle: &Puzzle) -> u32 {
    let (w, h) = puzzle.map.size();
    let mut total = 0;
    for y in 0..h {
        for x in 0..w {
            total += puzzle.score(x, y);
        }
    }
    return total;
}

fn part_2(puzzle: &Puzzle) -> u32 {
    let (w, h) = puzzle.map.size();
    let mut total = 0;
    for y in 0..h {
        for x in 0..w {
            total += puzzle.rating(x, y);
        }
    }
    return total;
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_10/input.txt");
    let data = Puzzle::load(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_10/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 36);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_10/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_2(&puzzle), 81);
    }
}
