use std::collections::HashSet;
use crate::aoc::grid::Grid;

const DBG: bool = cfg!(debug_assertions);

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

#[derive(Debug)]
struct Region {
    name: char,
    area: u32,
    perimeter: u32,
    edges: u32
}

fn get_regions(puzzle: &Puzzle) -> Vec<Region> {
    let (iw, ih) = {let (w, h) = puzzle.map.size(); (w as i32, h as i32)};

    struct R {
        name: char,
        cells: Vec<(i32, i32)>,
        borders: Vec<(char, i32, i32)>
    }

    let mut rs = Vec::new();
    let mut vs = HashSet::new();
    for y in 0..ih {
        for x in 0..iw {
            let n = puzzle.map.get((x as usize, y as usize));
            if !vs.contains(&(x, y)) {
                vs.insert((x, y));
                let mut r = R {name: n, cells: Vec::new(), borders: Vec::new()};
                let mut s = Vec::new();
                s.push((x, y));
                while !s.is_empty() {
                    let (x, y) = s.pop().unwrap();
                    r.cells.push((x, y));
                    for d in ['<', '>', '^', 'v'] {
                        let (dx, dy) = get_offset(d);
                        let (nx, ny) = (x + dx, y + dy);
                        if (nx >= 0) && (nx < iw) && (ny >= 0) && (ny < ih) && (puzzle.map.get((nx as usize, ny as usize)) == n) {
                            if !vs.contains(&(nx, ny)) {
                                vs.insert((nx, ny));
                                s.push((nx, ny));
                            }
                        } else {
                            r.borders.push((d, x, y));
                        }
                    }
                }
                rs.push(r);
            }
        }
    }

    let mut rs:Vec<_> = rs.iter_mut().map(|r| {
        let name = r.name;
        let area = r.cells.len() as u32;
        let perimeter = r.borders.len() as u32;
        let edges = ['<', '>', '^', 'v'].iter().map(|d| {
            let mut count = 0;
            let (dx, dy) = get_offset(*d);
            let (dx, dy) = (dy.abs(), dx.abs());
            if dx == 0 {
                r.borders.sort();
            } else {
                r.borders.sort_by(|(_, ax, ay), (_, bx, by)| (d, ay, ax).cmp(&(d, by, bx)));
            }
            let mut bs = r.borders.iter().filter(|b| b.0 == *d).peekable();
            while let Some((_, x, y)) = bs.next() {
                count += 1;
                let (mut x, mut y) = (*x, *y);
                while let Some((_, nx, ny)) = bs.peek() {
                    x += dx;
                    y += dy;
                    if (x != *nx) || (y != *ny) {break;}
                    bs.next();
                }
            }
            count as u32
        }).sum();
        Region {name, area, perimeter, edges}
    }).collect();
    if DBG {
        rs.sort_by(|r1, r2| {
            (r1.name, r1.area, r1.perimeter, r1.edges).cmp(&(r2.name, r2.area, r2.perimeter, r2.edges))
        });
    }
    rs
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

fn part_1(puzzle: &Puzzle) -> u32 {
    get_regions(puzzle).iter().map(|r| r.area * r.perimeter).sum()
}

fn part_2(puzzle: &Puzzle) -> u32 {
    get_regions(puzzle).iter().map(|r| r.area * r.edges).sum()
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_12/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    // 140 / 80
    const TEST_ABCDE: &str = "
    AAAA
    BBCD
    BBCC
    EEEC
    ";

    // 772 / 436
    const TEST_OX: &str = "
    OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO
    ";

    #[test]
    fn test_regions() {
        let puzzle = Puzzle::load(TEST_ABCDE);
        let regions = get_regions(&puzzle);
        for r in regions {
            let (area, perimeter, edges) = match r.name {
                'A' => (4, 10, 4),
                'B' => (4, 8, 4),
                'C' => (4, 10, 8),
                'D' => (1, 4, 4),
                'E' => (3, 8, 4),
                _ => panic!("invalid factory")
            };
            assert_eq!(r.area, area);
            assert_eq!(r.perimeter, perimeter);
            assert_eq!(r.edges, edges);
        }

        let puzzle = Puzzle::load(TEST_OX);
        let regions = get_regions(&puzzle);
        for r in regions {
            let (area, perimeter, edges) = match r.name {
                'O' => (21, 36, 20),
                'X' => (1, 4, 4),
                _ => panic!("invalid factory")
            };
            assert_eq!(r.area, area);
            assert_eq!(r.perimeter, perimeter);
            assert_eq!(r.edges, edges);
        }
    }

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(TEST_ABCDE);
        assert_eq!(part_1(&puzzle), 140);

        let puzzle = Puzzle::load(TEST_OX);
        assert_eq!(part_1(&puzzle), 772);

        let data = include_str!("../../data/day_12/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 1930);
    }

    #[test]
    fn test_part_2() {
        let data = Puzzle::load(TEST_ABCDE);
        assert_eq!(part_2(&data), 80);

        let puzzle = Puzzle::load(TEST_OX);
        assert_eq!(part_2(&puzzle), 436);

        let data = include_str!("../../data/day_12/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_2(&puzzle), 1206);
    }
}
