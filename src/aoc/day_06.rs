use crate::aoc::grid::Grid;

#[derive(Eq, PartialEq)]
enum Step {
    New,
    Visited,
    Loop,
    Out
}

type Position = (i32, i32, char);

#[derive(Clone)]
struct Puzzle {
    map: Grid<char>,
    position: Option<Position>
}

impl Puzzle {
    fn new(data: &str) -> Self {
        let map = data.trim().lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let map = Grid::from_vec(&map);
        let position = map.find_by(|c| "^v<>".contains(*c));
        let position = position.and_then(|(x, y)| Some((x as i32, y as i32, map.get((x, y)))));
        Self {map, position}
    }

    fn next(&self) -> Option<Position> {
        self.position.and_then(|(x, y, d)| {
            let (nx, ny) = match d {
                '^' => (x, y - 1),
                '>' => (x + 1, y),
                'v' => (x, y + 1),
                '<' => (x - 1, y),
                _ => panic!("invalid direction")
            };
            let (sx, sy) = self.map.size();
            let (sx, sy) = (sx as i32, sy as i32);
            if (nx >= 0) && (nx < sx) && (ny >= 0) && (ny < sy) {
                let c = self.map.get((nx as usize, ny as usize));
                if c == '#' {
                    let nd = match d {
                        '^' => '>',
                        '>' => 'v',
                        'v' => '<',
                        '<' => '^',
                        _ => panic!("invalid direction")
                    };
                    Some((x, y, nd))
                } else {
                    Some((nx, ny, d))
                }
            } else {
                None
            }
        })
    }

    fn step(&mut self) -> Step {
        let position = self.next();
        match position {
            Some((nx, ny, nd)) => {
                self.position = position;
                let c = self.map.get((nx as usize, ny as usize));
                if c == '.' {
                    self.map.set((nx as usize, ny as usize), nd);
                    Step::New
                } else if c == nd {
                    Step::Loop
                } else {
                    Step::Visited
                }
            },
            None => {
                self.position = None;
                Step::Out
            }
        }
    }

    fn reset(&mut self, position: Option<Position>) {
        let (w, h) = self.map.size();
        for y in 0..h {
            for x in 0..w {
                let p = (x, y);
                if self.map.get(p) != '#' {
                    self.map.set(p, '.');
                }
            }
        }
        if let Some((x, y, d)) = position {
            self.map.set((x as usize, y as usize), d);
        }
        self.position = position;
    }
}

fn part_1(puzzle: &mut Puzzle) -> u32 {
    let mut total = 1u32;
    while !puzzle.position.is_none() {
        if puzzle.step() == Step::New {total += 1;}
    }
    total
}

fn part_2(puzzle: &mut Puzzle) -> u32 {
    let mut total = 0;
    while !puzzle.position.is_none() {
        let mut test = puzzle.clone();
        if let Some((x, y, _)) = test.next() {
            if test.map.get((x as usize, y as usize)) == '.' {
                test.map.set((x as usize, y as usize), '#');
                while !test.position.is_none() {
                    if test.step() == Step::Loop {
                        total += 1;
                        break;
                    }
                }
            }
        }
        puzzle.step();
    }
    total
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_06/input.txt");
    let mut puzzle = Puzzle::new(data);
    let start = puzzle.position;
    println!("part 1: {}", part_1(&mut puzzle));
    puzzle.reset(start);
    println!("part 2: {}", part_2(&mut puzzle));
}

mod tests {
    use super::*;

    const DATA: &str = include_str!("../../data/day_06/test.txt");

    #[test]
    fn test_part_1() {
        let mut puzzle = Puzzle::new(DATA);
        assert_eq!(part_1(&mut puzzle), 41);
    }

    #[test]
    fn test_part_2() {
        let mut puzzle = Puzzle::new(DATA);
        assert_eq!(part_2(&mut puzzle), 6);
    }
}
