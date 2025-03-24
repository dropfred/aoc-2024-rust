// use std::collections::VecDeque;
use std::collections::HashSet;

struct Data {
    grid: Vec<Vec<char>>
}

impl Data {
    fn parse(data: &str) -> Self {
        let grid = data.trim().lines().map(|s| {
            s.trim().chars().collect()
        }).collect();
        Data {grid}
    }

    fn get(&self, p: &Position) -> char {
        self.grid[p.y as usize][p.x as usize]
    }

    // fn get_mut(&mut self, x: i32, y: i32) -> &mut char {
    //     &mut self.grid[y as usize][x as usize]
    // }

    fn size(&self) -> (i32, i32) {
        (self.grid[0].len() as i32, self.grid.len() as i32)
    }

    fn find(&self, c: char) -> Option<Position> {
        let (w, h) = self.size();
        for y in 1..(h - 1) {
            for x in 1..(w - 1) {
                let p = Position {x, y};
                if self.get(&p) == c {
                    return 
                    Some(p);
                }
            }
        }
        None
    }
}

fn get_next_position(p: &Position, d: char) -> Position {
    let (dx, dy) = match d {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => panic!("invalid move")
    };
    Position {x: p.x + dx, y: p.y + dy}
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut nl = false;
        for r in &self.grid {
            if nl {write!(f, "\n")?;}
            for c in r {
                write!(f, "{}", c)?;
            }
            nl = true;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32
}
#[derive(Debug)]
struct Tile {
    p: Position,
    d: char,
    t: u32
}

fn solve_maze(data: &Data) -> Option<u32> {
    let mut ps = Vec::new();
    let mut vs = HashSet::new();
    ps.push(Tile {p: data.find('S').expect("start"), d: '>', t: 0});
    while !ps.is_empty() {
        let Tile {p, d, t} = ps.pop().unwrap();
        if data.get(&p) == 'E' {
            return Some(t);
        }
        vs.insert((p.x, p.y, d));
        let np = get_next_position(&p, d);
        if !vs.contains(&(np.x, np.y, d)) && data.get(&np) != '#' {
            ps.push(Tile {p: np, d, t: t + 1});
        }
        let nds = if (d == '<') || (d == '>') {('^', 'v')} else {('<', '>')};
        let np = get_next_position(&p, nds.0);
        if !vs.contains(&(np.x, np.y, d)) && data.get(&np) != '#' {
            ps.push(Tile {p: np, d: nds.0, t: t + 1001});
        }
        let np = get_next_position(&p, nds.1);
        if !vs.contains(&(np.x, np.y, d)) && data.get(&np) != '#' {
            ps.push(Tile {p: np, d: nds.1, t: t + 1001});
        }
        ps.sort_by(|a, b| b.t.cmp(&a.t));
    }
    None
}

fn part_1(data: &Data) -> u32 {
    solve_maze(data).unwrap()
}

fn part_2(data: &Data) -> u32 {
    todo!("part 2");
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_16/test_1.txt");
    let data = Data::parse(data);
    println!("{data}");

    let data = include_str!("../../data/day_16/input.txt");
    let data = Data::parse(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_16/test_1.txt");
        let data = Data::parse(data);
        let size = data.size();
        assert!((size.0 == 15) && (size.1 == 15));

        let data = include_str!("../../data/day_16/test_2.txt");
        let data = Data::parse(data);
        let size = data.size();
        assert!((size.0 == 17) && (size.1 == 17));
    }

    #[test]
    fn test_part_1_0() {
        let data = "
        ####
        #SE#
        ####
        ";
        let data = Data::parse(data);
        assert_eq!(part_1(&data), 1);

        let data = "
        #####
        #S.E#
        #####
        ";
        let data = Data::parse(data);
        assert_eq!(part_1(&data), 2);

        let data = "
        ###
        #E#
        #S#
        ###
        ";
        let data = Data::parse(data);
        assert_eq!(part_1(&data), 1001);

        let data = "
        ###
        #E#
        #.#
        #S#
        ###
        ";
        let data = Data::parse(data);
        assert_eq!(part_1(&data), 1002);
    }

    #[test]
    fn test_part_1_1() {
        let data = include_str!("../../data/day_16/test_1.txt");
        let data = Data::parse(data);
        assert_eq!(part_1(&data), 7036);
    }

    #[test]
    fn test_part_1_2() {
        let data = include_str!("../../data/day_16/test_2.txt");
        let data = Data::parse(data);
        assert_eq!(part_1(&data), 11048);
    }

    #[test]
    fn test_part_2_1() {
        let data = include_str!("../../data/day_16/test_1.txt");
        let data = Data::parse(data);
        assert_eq!(part_2(&data), 45);
    }

    #[test]
    fn test_part_2_2() {
        let data = include_str!("../../data/day_16/test_2.txt");
        let data = Data::parse(data);
        assert_eq!(part_2(&data), 64);
    }
}
