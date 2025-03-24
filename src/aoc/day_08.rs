use std::collections::{HashMap, HashSet};
use crate::aoc::grid::Grid;

struct Puzzle {
    map: Grid<char>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let map = Grid::parse(data, "")?;
        Some(Self {map})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn solve_part_1(puzzle: &Puzzle) -> usize {
    let (width, height) = puzzle.map.size();
    let (width, height) = (width as i32, height as i32);
    let mut antennas = HashMap::new();
    let mut anti_nodes = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            let c = puzzle.map.get((x as usize, y as usize));
            if c != '.' {
                let antennas = antennas.entry(c).or_insert(Vec::new());
                for (ax, ay) in antennas.iter() {
                    let (dx, dy) = (x - ax, y - ay);
                    let (nx, ny) = (ax - dx, ay - dy);
                    if (nx >= 0) && (nx < width) && (ny >= 0) && (ny < height) {
                        anti_nodes.insert((nx, ny));
                    }
                    let (nx, ny) = (x + dx, y + dy);
                    if (nx >= 0) && (nx < width) && (ny >= 0) && (ny < height) {
                        anti_nodes.insert((nx, ny));
                    }
                }
                antennas.push((x, y));
            }
        }
    }
    anti_nodes.len()
}

fn solve_part_2(puzzle: &Puzzle) -> usize {
    let (width, height) = puzzle.map.size();
    let (width, height) = (width as i32, height as i32);
    let mut antennas = HashMap::new();
    let mut anti_nodes = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            let c = puzzle.map.get((x as usize, y as usize));
            if c != '.' {
                let antennas = antennas.entry(c).or_insert(Vec::new());
                for (ax, ay) in antennas.iter() {
                    let (dx, dy) = (x - ax, y - ay);
                    for i in 0.. {
                        let (nx, ny) = (ax - dx * i, ay - dy * i);
                        let d1 = if (nx >= 0) && (nx < width) && (ny >= 0) && (ny < height) {
                            anti_nodes.insert((nx, ny));
                            false
                        } else {
                            true
                        };
                        let (nx, ny) = (x + dx * i, y + dy * i);
                        let d2 = if (nx >= 0) && (nx < width) && (ny >= 0) && (ny < height) {
                            anti_nodes.insert((nx, ny));
                            false
                        } else {
                            true
                        };
                        if d1 && d2 {break;}
                    }
                }
                antennas.push((x, y));
            }
        }
    }
    anti_nodes.len()
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_08/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_08/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(solve_part_1(&puzzle), 14);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_08/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(solve_part_2(&puzzle), 34);
    }
}
