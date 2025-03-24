use std::collections::{HashSet, VecDeque};

struct Grid {
    cells: Vec<Vec<char>>
}

impl Grid {
    fn new(w: usize, h: usize, e: char) -> Self {
        assert!((w > 0) && (h > 0));
        let mut cells = Vec::with_capacity(h);
        for _ in 0..h {
            cells.push(vec![e;w]);
        }
        Self {cells}
    }

    fn parse(data: &str) -> Option<Self> {
        let cells: Vec<Vec<_>> = data.trim().lines().map(|r| {
            r.trim().chars().collect()
        }).collect();
        if cells.is_empty() || cells.iter().any(|r| {r.is_empty() || (r.len() != cells[0].len())}) {
            return None;
        }
        Some(Self {cells})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }

    fn size(&self) -> (usize, usize) {
        (self.cells[0].len(), self.cells.len())
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.cells[y][x]
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        self.cells[y][x] = c;
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut nl = false;
        for r in &self.cells {
            if nl {
                writeln!(f, "")?;
            }
            for c in r {
                write!(f, "{}", c)?;
            }
            nl = true;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Puzzle {
    bytes: Vec<(u32, u32)>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let bytes: Vec<_> = data.trim().lines().map(|s| {
            let (x, y) = s.trim().split_once(',').unwrap();
            let (x, y) = (x.parse::<u32>(), y.parse::<u32>());
            (x, y)
            
        }).collect();
        if bytes.iter().any(|(x, y)| x.is_err() || y.is_err()) {
            return None;
        }
        let bytes = bytes.into_iter().map(|(x, y)| {(x.unwrap(), y.unwrap())}).collect();
        Some(Puzzle {bytes: bytes})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn get_offset(d: char) -> (i32, i32) {
    match d {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => panic!("invalid direction")
    }
}

fn solve_maze(map: &Grid, entry: (usize, usize), exit: (usize, usize), wall: char) -> Option<u32> {
    let (width, height) = {let (w, h) = map.size(); (w as i32, h as i32)};
    let entry = (entry.0 as i32, entry.1 as i32);
    let exit = (exit.0 as i32, exit.1 as i32);
    let mut ps = VecDeque::new();
    let mut vs = HashSet::new();
    let mut visit = |ps: &mut VecDeque<_>, p, s: u32| {
        if !vs.contains(&p) {
            let (x, y) = p;
            if (x >= 0) && (x < width) && (y >= 0) && (y < height) && (map.get(x as usize, y as usize) != wall) {
                vs.insert(p);
                ps.push_back((s, p));
            }
        }
    };
    visit(&mut ps, entry, 0);
    while !ps.is_empty() {
        let (s, p) = ps.pop_front().unwrap();
        if p == exit {
            return Some(s);
        }
        let (x, y) = p;
        for d in ['<', '>', '^', 'v'] {
            let (dx, dy) = get_offset(d);
            let (nx, ny) = (x + dx, y + dy);
            visit(&mut ps, (nx, ny), s + 1);
        }
    }
    None
}

fn solve_part_1(puzzle: &Puzzle, w: usize, h: usize, n: usize) -> Option<u32> {
    let mut memory = Grid::new(w, h, '.');
    for (x, y) in puzzle.bytes.iter().take(n) {
        memory.set(*x as usize, *y as usize, 'X');
    }
    solve_maze(&memory, (0, 0), (w - 1, h - 1), 'X')

}

fn solve_part_2(puzzle: &Puzzle, w: usize, h: usize, skip: usize) -> Option<(u32, u32)> {
    let mut b: usize = skip;
    let mut e = puzzle.bytes.len() - 1;
    loop {
        let m = (b + e) / 2;
        let ms = solve_part_1(&puzzle, w, h, m).is_some();
        let mn = solve_part_1(&puzzle, w, h, m + 1).is_none();
        if ms && mn {
            return Some(puzzle.bytes[m]);
        }
        if (e - b) > 1 {
            if ms {
                b = m;
            } else {
                e = m;
            }
        } else {
            break;
        }
    }
    None
}

fn part_1(puzzle: &Puzzle) -> u32 {
    solve_part_1(puzzle, 71, 71, 1024).unwrap()
}

fn part_2(puzzle: &Puzzle) -> (u32, u32) {
    solve_part_2(puzzle, 71, 71, 1024).unwrap()
}

pub(crate) fn solve() {
    let puzzle = include_str!("../../data/day_18/input.txt");
    let puzzle = Puzzle::load(puzzle);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {:?}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let puzzle = include_str!("../../data/day_18/test.txt");
        let puzzle = Puzzle::load(puzzle);
        assert_eq!(solve_part_1(&puzzle, 7, 7, 12).unwrap(), 22);
    }

    #[test]
    fn test_part_2() {
        let puzzle = include_str!("../../data/day_18/test.txt");
        let puzzle = Puzzle::load(puzzle);
        assert_eq!(solve_part_2(&puzzle, 7, 7, 12).unwrap(), (6, 1));
    }
}
