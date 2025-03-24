use crate::aoc::grid::Grid;

struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32
}
struct Puzzle {
    robots: Vec<Robot>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_robot = |s: &str| {
            let (p, v) = s.split_once(" ")?;
            let (px, py) = p.trim_start_matches("p=").split_once(",")?;
            let (vx, vy) = v.trim_start_matches("v=").split_once(",")?;
            let (px, py) = (px.parse().ok()?, py.parse().ok()?);
            let (vx, vy) = (vx.parse().ok()?, vy.parse().ok()?);
            Some(Robot {px, py, vx, vy})
        };
        let robots: Option<_> = data.trim().lines().map(parse_robot).collect();
        let robots = robots?;
        Some(Puzzle {robots})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn quadrants(puzzle: &Puzzle, size: (i32, i32), time: i32) -> u32 {
    let (w, h) = size;
    let (w2, h2) = (w / 2, h / 2);
    let qs = puzzle.robots.iter().map(|r| {
        let (mut x, mut y) = ((r.px + r.vx * time) % w, (r.py + r.vy * time) % h);
        if x < 0 {x += w;}
        if y < 0 {y += h;}
        (x, y)
    }).filter(|(x, y)| {
        (*x != w2) && (*y != h2)
    }).map(|(x, y)| {
        ((x >= w2) as u32, (y >= h2) as u32)
    }).fold((0, 0, 0, 0), |(q00, q01, q10, q11), q| {
        match q {
            (0, 0) => (q00 + 1, q01, q10, q11),
            (0, 1) => (q00, q01 + 1, q10, q11),
            (1, 0) => (q00, q01, q10 + 1, q11),
            (1, 1) => (q00, q01, q10, q11 + 1),
            _ => panic!("invalid quadrant")
        }
    });
    qs.0 * qs.1 * qs.2 * qs.3
}

fn part_1(puzzle: &Puzzle) -> u32 {
    quadrants(puzzle, (101, 103), 100)
}

fn print_robots(robots: &Vec<Robot>, size: (usize, usize), time: u32) {
    let (w, h) = size;
    let (w, h) = (w as i32, h as i32);
    let mut map = Grid::new(size, ' ');
    for p in robots.iter().map(|r| {
        let (mut x, mut y) = ((r.px + r.vx * time as i32) % w, (r.py + r.vy * time as i32) % h);
        if x < 0 {x += w;}
        if y < 0 {y += h;}
        (x as usize, y as usize)
    }) {
        map.set(p, '#');
    }
    println!("{map}");
}

use std::collections::HashMap;

fn part_2(puzzle: &Puzzle, size: (usize, usize)) -> u32 {
    let n = puzzle.robots.len();
    let (w, h) = size;
    let (w, h) = (w as i32, h as i32);
    // assume that at least half of the robots are in the middle of the map
    let (w4, h4) = (w / 4, h / 4);
    for t in 0.. {
        if puzzle.robots.iter().map(|r| {
            let mut x = (r.px + r.vx * t) % w;
            let mut y = (r.py + r.vy * t) % h;
            if x < 0 {x += w;}
            if y < 0 {y += h;}
            (x, y)
        }).filter(|(x, y)| {
            (*x > w4) && (*x < (w - w4)) && (*y > h4) && (*y < (h - h4))
        }).count() > (n / 2) {
            print_robots(&puzzle.robots, size, t as u32);
            return t as u32;
        }
    }

    // assume that robots are clumped (very crude approximation)
    // for t in 0.. {
    //     let mut map = HashMap::new();
    //     for (x, y) in puzzle.robots.iter().map(|r| {
    //         let mut x = (r.px + r.vx * t) % w;
    //         let mut y = (r.py + r.vy * t) % h;
    //         if x < 0 {x += w;}
    //         if y < 0 {y += h;}
    //         (x / 3, y / 3)
    //     }) {
    //         let n = map.entry((x as usize, y as usize)).or_insert(0usize);
    //         *n += 1;
    //     }
    //     let ns = map.values().sum::<usize>() as f32 / map.len() as f32;
    //     // println!("{ns}");
    //     if ns > 2.0 {
    //         print_robots(&puzzle.robots, size, t as u32);
    //         return t as u32;
    //     }

    // }
    0
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_14/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle, (101, 103)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_14/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(puzzle.robots.len(), 12)
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_14/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(quadrants(&puzzle, (11, 7), 100), 12);
    }
}
