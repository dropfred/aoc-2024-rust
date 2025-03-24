use crate::aoc::grid::{Grid, GridExploreIterator};

pub type MazeExploreIterator<'a, F>  = GridExploreIterator<'a, char, F>;

pub struct MazePathIterator {
    path: Vec<(usize, usize)>
}

impl Iterator for MazePathIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.path.pop()
    }
}

pub struct Maze {
    map: Grid<char>
}

impl Maze {
    pub fn parse(data: &str) -> Option<Self> {
        Some(Self {map: data.parse().ok()?})
    }

    pub fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }

    pub fn get_map(&self) -> &Grid<char> {
        &self.map
    }

    pub fn explore(
        &self,
        start: (usize, usize),
        wall: char
    ) -> MazeExploreIterator<impl FnMut((usize, usize), (usize, usize), usize) -> bool + '_> {
        MazeExploreIterator::new(self.get_map(), start, move |p, _, _| self.get_map().get(p) != wall)
    }

    pub fn get_path(
        &self, begin: (usize, usize),
        end: (usize, usize),
        wall: char
    ) -> Option<MazePathIterator> {
        let mut pps:Grid<(usize, usize)> = Grid::new(self.map.size(), (0, 0));
        for (p, pp, _) in self.explore(begin, wall) {
            pps.set(p, pp);
            if p == end {
                let mut path = Vec::new();
                let mut p = p;
                loop {
                    path.push(p);
                    if p == begin {
                        break;
                    }
                    p = pps.get(p);
                }
                return Some(MazePathIterator {path});
            }
        }
        None
    }

    pub fn get_distance(
        &self,
        begin: (usize, usize),
        end: (usize, usize),
        wall: char
    ) -> Option<usize> {
        for (p, _, d) in self.explore(begin, wall) {
            if p == end {
                return Some(d);
            }
        }
        None
    }
}

impl std::str::FromStr for Maze {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.parse();
        match map {
            Ok(map) => Ok(Maze {map}),
            Err(_) => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "
        ###
        #.#
        ###
        ";
        let maze = Maze::load(data);
        assert_eq!(maze.map.size(), (3, 3));
    }

    #[test]
    fn test_explore() {
        let data = "
        #####
        #...#
        #####
        ";
        let maze = Maze::load(data);
        assert_eq!(maze.explore((2, 1), '#').count(), 3);
        assert_eq!(maze.explore((2, 1), '.').count(), 0);
    }

    #[test]
    fn test_get_distance() {
        let data = "
        #####
        #B#E#
        #...#
        #####
        ";
        let maze = Maze::load(data);
        let map = &maze.get_map();
        assert_eq!(maze.get_distance(map.find('B').unwrap(), map.find('E').unwrap(), '#').unwrap(), 4);
        assert_eq!(maze.get_distance(map.find('B').unwrap(), map.find('E').unwrap(), 'X').unwrap(), 2);

        let data = "
        #######
        #B###E#
        #.#.#.#
        #.....#
        #######
        ";
        let maze = Maze::load(data);
        let map = &maze.get_map();
        assert_eq!(maze.get_distance(map.find('B').unwrap(), map.find('E').unwrap(), '#').unwrap(), 8);

        let data = "
        #########
        #B....#.#
        #####...#
        #E......#
        #########
        ";
        let maze = Maze::load(data);
        let begin = maze.get_map().find('B').unwrap();
        let end = maze.get_map().find('E').unwrap();
        assert_eq!(maze.get_distance(begin, end, '#'), Some(10));
    }

    #[test]
    fn test_get_path() {
        let data = "
        ####
        #BE#
        ####
        ";
        let maze = Maze::load(data);
        let map = &maze.get_map();
        let mut path = maze.get_path(map.find('B').unwrap(), map.find('E').unwrap(), '#').unwrap();
        assert_eq!(path.next(), Some((1, 1)));
        assert_eq!(path.next(), Some((2, 1)));
        assert_eq!(path.next(), None);
    }
}
