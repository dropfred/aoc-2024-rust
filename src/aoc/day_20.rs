use std::collections::HashMap;

use crate::aoc::{grid::Grid, maze::Maze};

struct Puzzle {
    maze: Maze
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        Some(Self {maze: Maze::parse(data)?})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn get_cheats(maze: &Maze, cheat: usize, save: usize) -> Option<Vec<(usize, ((usize, usize), (usize, usize)))>> {
    let map = maze.get_map();
    let begin = map.find('S')?;
    let end = map.find('E')?;

    let mut distances = Grid::new(maze.get_map().size(), usize::MAX);
    for (p, _, d) in maze.explore(end, '#') {
        distances.set(p, d);
    }
    let distance = distances.get(begin);

    let mut cheats = HashMap::new();
    for (d, p) in maze.get_path(begin, end, '#')?.enumerate() {
        for (cp, _, cd) in map.explore(p, |_, _, _| true).skip(1) {
            if cd > cheat {break;}
            if (cd == 1) && map.get(cp) != '#' {continue;}
            if !cheats.contains_key(&(p, cp)) {
                let de = distances.get(cp);
                if de != usize::MAX {
                    let d = d + cd + de;
                    if distance > d {
                        cheats.insert((p, cp), distance - d);
                    }
                }
            }
        }
    }
    let mut cheats: Vec<_> = cheats.into_iter()
        .filter(|(_, s)| *s >= save)
        .map(|(pcp, s)| (s, pcp))
        .collect();
    cheats.sort();
    Some(cheats)
}

fn solve_part_1(puzzle: &Puzzle) -> usize {
    let cheats = get_cheats(&puzzle.maze, 2, 100).expect("solvable puzzle");
    cheats.len()
}

fn solve_part_2(puzzle: &Puzzle) -> usize {
    let cheats = get_cheats(&puzzle.maze, 20, 100).expect("solvable puzzle");
    cheats.len()

}

pub(crate) fn solve() {
    let puzzle = include_str!("../../data/day_20/input.txt");
    let puzzle = Puzzle::load(puzzle);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {:?}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_get_cheats() {
        let puzzle = include_str!("../../data/day_20/test.txt");
        let puzzle = Puzzle::load(puzzle);
        let maze = &puzzle.maze;

        let cheats = get_cheats(maze, 2, 1).unwrap();
        let mut save = HashMap::new();
        for (s, _) in cheats {
            let ts = save.entry(s).or_insert(0);
            *ts += 1;
        }
        let mut save: Vec<_> = save.into_iter().map(|(s, t)| (s, t)).collect();
        save.sort();
        let mut i = save.into_iter();
        assert_eq!(i.next(), Some((2, 14)));
        assert_eq!(i.next(), Some((4, 14)));
        assert_eq!(i.next(), Some((6, 2)));
        assert_eq!(i.next(), Some((8, 4)));
        assert_eq!(i.next(), Some((10, 2)));
        assert_eq!(i.next(), Some((12, 3)));
        assert_eq!(i.next(), Some((20, 1)));
        assert_eq!(i.next(), Some((36, 1)));
        assert_eq!(i.next(), Some((38, 1)));
        assert_eq!(i.next(), Some((40, 1)));
        assert_eq!(i.next(), Some((64, 1)));
        assert_eq!(i.next(), None);

        let cheats = get_cheats(maze, 20, 50).unwrap();
        let mut save = HashMap::new();
        for (s, _) in cheats {
            let ts = save.entry(s).or_insert(0);
            *ts += 1;
        }
        let mut save: Vec<_> = save.into_iter().map(|(s, t)| (s, t)).collect();
        save.sort();
        let mut i = save.into_iter();
        assert_eq!(i.next(), Some((50, 32)));
        assert_eq!(i.next(), Some((52, 31)));
        assert_eq!(i.next(), Some((54, 29)));
        assert_eq!(i.next(), Some((56, 39)));
        assert_eq!(i.next(), Some((58, 25)));
        assert_eq!(i.next(), Some((60, 23)));
        assert_eq!(i.next(), Some((62, 20)));
        assert_eq!(i.next(), Some((64, 19)));
        assert_eq!(i.next(), Some((66, 12)));
        assert_eq!(i.next(), Some((68, 14)));
        assert_eq!(i.next(), Some((70, 12)));
        assert_eq!(i.next(), Some((72, 22)));
        assert_eq!(i.next(), Some((74, 4)));
        assert_eq!(i.next(), Some((76, 3)));
    }
}
