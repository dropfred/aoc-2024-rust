use crate::aoc::grid::Grid;

struct Puzzle {
    map: Grid<char>,
    moves: Vec<char>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let data = data.trim().replace("\r", "");
        let (map, moves) = data.split_once("\n\n")?;
        let map = Grid::parse(map, "")?;
        let moves = moves.lines().map(|s| s.trim()).collect::<String>().chars().collect();
        Some(Puzzle {map, moves})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> u32 {
    let mut map = puzzle.map.clone();

    let (x, y) = puzzle.map.find('@').expect("robot");
    let (mut x, mut y) = (x as i32, y as i32);

    for m in &puzzle.moves {
        let (dx, dy) = match m {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("invalid move")
        };
        let (mut nx, mut ny) = (x + dx, y + dy);
        while map.get((nx as usize, ny as usize)) == 'O' {
            nx += dx;
            ny += dy;
        }
        if map.get((nx as usize, ny as usize)) == '.' {
            let d = (nx - x).abs() + (ny - y).abs();
            for _ in 0..d {
                let c = map.get(((nx - dx) as usize, (ny - dy) as usize));
                map.set((nx as usize, ny as usize), c);
                nx -= dx;
                ny -= dy;
            }
            map.set((x as usize, y as usize), '.');
            x += dx; y += dy;
        }
    }
    let mut total = 0;

    let (w, h) = puzzle.map.size();
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if map.get((x, y)) == 'O' {
                total += x + 100 * y;
            }
        }
    }

    total as u32
}

fn part_2(data: &Puzzle) -> u32 {
    todo!("part 2");
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_15/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let data = include_str!("../../data/day_15/test_1.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 2028);
    }

    #[test]
    fn test_part_1_2() {
        let data = include_str!("../../data/day_15/test_2.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 10092);
    }
}
