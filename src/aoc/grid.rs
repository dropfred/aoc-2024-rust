use std::collections::VecDeque;
use std::cmp::PartialEq;

#[derive(Clone)]
pub struct Grid<T> {
    size: (usize, usize),
    data: Vec<T>
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn new(size: (usize, usize), e: T) -> Self {
        let s = size.0 * size.1;
        assert!(s > 0);
        let data = vec![e; s];
        Self {size, data}
    }

    pub fn from_vec(data: &Vec<Vec<T>>) -> Self {
        assert!(
            !data.is_empty() &&
            !data[0].is_empty() &&
            data.iter().skip(1).all(|r| r.len() == data[0].len())
        );
        let (width, height) = (data[0].len(), data.len());
        let data = data.iter().flat_map(|xs| xs.iter().cloned()).collect();
        Self {size: (width, height), data}
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn get(&self, point: (usize, usize)) -> T {
        self.data[point.1 * self.size.0 + point.0]
    }

    pub fn set(&mut self, point: (usize, usize), v: T) {
        self.data[point.1 * self.size.0 + point.0] = v;
    }

    pub fn find(&self, v: T) -> Option<(usize, usize)> {
        let (w, h) = self.size();
        for y in 0..h {
            for x in 0..w {
                if self.get((x, y)) == v {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn find_by<F: FnMut(&T) -> bool>(&self, mut f: F) -> Option<(usize, usize)> {
        let (w, h) = self.size();
        for y in 0..h {
            for x in 0..w {
                if f(&self.get((x, y))) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    // pub fn explore<'a, F> (&'a self, start: (usize, usize), filter: F)
    // -> impl Iterator<Item = ((usize, usize), (usize, usize), usize)> + 'a
    // where F: FnMut((usize, usize), (usize, usize), usize) -> bool + 'a
    // {
    //         GridExploreIterator::new(self, start, filter)
    // }

    pub fn explore<F> (&self, start: (usize, usize), filter: F) -> GridExploreIterator<T, F>
    where F: FnMut((usize, usize), (usize, usize), usize) -> bool
    {
        GridExploreIterator::new(self, start, filter)
    }

    pub fn cells(&self) -> impl Iterator<Item = ((usize, usize), T)> + '_ {
        self.data.iter().enumerate().map(|(i, c)| {
            let w = self.size.0;
            let x = i % w;
            let y = i / w;
            ((x, y), *c)
        })
    }

    pub fn row(&self, index: usize) -> impl Iterator<Item = T> + '_ {
        let mut i = 0;
        std::iter::from_fn(move || {
            if i < self.size().0 {
                let r = self.get((i, index));
                i += 1;
                Some(r)
            } else {
                None
            }
        })
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = T> + '_> {
        let mut i = 0;
        std::iter::from_fn(move || {
            if i < self.size().0 {
                let r = self.row(i);
                i += 1;
                Some(r)
            } else {
                None
            }
        })
    }

    pub fn column(&self, index: usize) -> impl Iterator<Item = T> + '_ {
        let mut i = 0;
        std::iter::from_fn(move || {
            if i < self.size().1 {
                let r = self.get((index, i));
                i += 1;
                Some(r)
            } else {
                None
            }
        })
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = T> + '_> {
        let mut i = 0;
        std::iter::from_fn(move || {
            if i < self.size().0 {
                let r = self.column(i);
                i += 1;
                Some(r)
            } else {
                None
            }
        })
    }
}

impl<T: Default + Copy + PartialEq> Grid<T> {
    pub fn new_default(size: (usize, usize)) -> Self {
        Self::new(size, Default::default())
    }
}

impl<T: Copy + PartialEq + std::str::FromStr + std::fmt::Debug> Grid<T> {
    pub fn parse(data: &str, sep: &str) -> Option<Self> where <T as std::str::FromStr>::Err: std::fmt::Debug {
        let data: Result<Vec<Vec<_>>, _> = data.trim().lines().map(|r| {
            if sep.is_empty() {
                r.trim().chars().map(|c| T::from_str(&c.to_string())).collect()
            } else {
                r.trim().split(sep).map(|s| T::from_str(&s)).collect()
            }
        }).collect();
        let data = data.ok()?;
        if !data.is_empty() && !data[0].is_empty() && data.iter().skip(1).all(|r| r.len() == data[0].len()) {
            Some(Self::from_vec(&data))
        } else {
            None
        }
    }

    pub fn load(data: &str, sep: &str) -> Self where <T as std::str::FromStr>::Err: std::fmt::Debug {
        Self::parse(data, sep).expect("valid input")
    }
}

enum Dir {
    East,
    West,
    North,
    South
}

impl Dir {
    fn offset(&self) -> (i32, i32) {
        match self {
            Dir::East => (-1, 0),
            Dir::West => (1, 0),
            Dir::North => (0, -1),
            Dir::South => (0, 1)
        }
    }

    fn all() -> [Dir; 4] {
        [Dir::East, Dir::West, Dir::North, Dir::South]
    }
}

pub struct GridExploreIterator<'a, T, F: FnMut((usize, usize), (usize, usize), usize) -> bool> {
    grid: &'a Grid<T>,
    filter: F,
    positions: VecDeque<((usize, usize), (usize, usize), usize)>,
    visited: Vec<u64>
}

impl<'a, T: Copy + PartialEq, F: FnMut((usize, usize), (usize, usize), usize) -> bool> GridExploreIterator<'a, T, F> {
    pub fn new(grid: &'a Grid<T>, start: (usize, usize), filter: F) -> Self {
        let (w, h) = grid.size();
        let positions = VecDeque::new();
        let visited = vec![0; h * (w + 63) / 64];
        let mut it = GridExploreIterator {grid, filter, positions, visited};
        it.visit(start, start, 0);
        it
    }

    fn visit(&mut self, position: (usize, usize), pposition: (usize, usize), distance: usize) {
        let (w, _) = self.grid.size();
        let w64 = (w + 63) / 64;
        let (vx, vy) = (position.0 / 64, position.1);
        let bx = 1 << (position.0 % 64);
        let v = vy * w64 + vx;
        if (self.visited[v] & bx) == 0 {
            self.visited[v] |= bx;
            if (self.filter)(position, pposition, distance) {
                self.positions.push_back((position, pposition, distance));
            }
        }
    }
}

impl<'a, T: Copy + PartialEq, F: FnMut((usize, usize), (usize, usize), usize) -> bool> Iterator for GridExploreIterator<'a, T, F> {
    type Item = ((usize, usize), (usize, usize), usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((position, pposition, distance)) = self.positions.pop_front() {
            let (w, h) = {let (w, h) = self.grid.size(); (w as i32, h as i32)};
            let (x, y) = (position.0 as i32, position.1 as i32);
            for d in Dir::all() {
                let (dx, dy) = d.offset();
                let (x, y) = (x + dx, y + dy);
                if (x >= 0) && (x < w) && (y >= 0) && (y < h) {
                    let p = (x as usize, y as usize);
                    self.visit(p, position, distance + 1);
                }
            }
            Some((position, pposition, distance))
        } else {
            None
        }
    }
}

impl std::str::FromStr for Grid<char> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Option<Self> = Grid::parse(s, "");
        match grid {
            Some(grid) => Ok(grid),
            None => Err(())
        }
    }
}

impl std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut nl = false;
        for y in 0..(self.size.1) {
            if nl {
                writeln!(f, "")?;
            }
            for x in 0..(self.size.0) {
                write!(f, "{}", self.get((x, y)))?;
            }
            nl = true;
        }
        Ok(())
    }
}

// impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let mut nl = false;
//         for y in 0..(self.height) {
//             if nl {
//                 writeln!(f, "")?;
//             }
//             for x in 0..(self.width) {
//                 write!(f, "{}", self.get(x, y))?;
//             }
//             nl = true;
//         }
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "
        1 2 3
        4 5 6
        7 8 9
        ";
        let grid: Grid<u8> = Grid::load(data, " ");
        assert_eq!(grid.size(), (3, 3));
        assert_eq!(grid.get((0, 0)), 1);
        assert_eq!(grid.get((2, 2)), 9);

        let data = "
        abc
        def
        ";
        let grid: Grid<char> = Grid::load(data, "");
        assert_eq!(grid.size(), (3, 2));

        let data = "
        123
        456
        789
        ";
        let grid: Grid<u8> = Grid::load(data, "");
        assert_eq!(grid.size(), (3, 3));
        let grid: Grid<u32> = Grid::load(data, ",");
        assert_eq!(grid.size(), (1, 3));

        let data = "
        1X3
        456
        ";
        let grid: Option<Grid<u8>> = Grid::parse(data, "");
        assert!(grid.is_none());
    }

    #[test]
    fn test_explore() {
        let data = "
        #####
        #...#
        #####
        ";
        let grid: Grid<char> = Grid::load(data, "");
        assert_eq!(grid.explore((2, 1), |_, _, _| true).count(), 15);
        assert_eq!(grid.explore((2, 1), |p, _, _| grid.get(p) != '#').count(), 3);
    }

    #[test]
    fn test_cells() {
        let data = "
        12
        34
        ";
        let grid: Grid<char> = Grid::load(data, "");
        let mut cells = grid.cells();
        assert_eq!(cells.next(), Some(((0, 0), '1')));
        assert_eq!(cells.next(), Some(((1, 0), '2')));
        assert_eq!(cells.next(), Some(((0, 1), '3')));
        assert_eq!(cells.next(), Some(((1, 1), '4')));
        assert_eq!(cells.next(), None);
    }

    #[test]
    fn test_row() {
        let data = "
        12
        34
        ";
        let grid: Grid<char> = Grid::load(data, "");
        let mut row = grid.row(0);
        assert_eq!(row.next(), Some('1'));
        assert_eq!(row.next(), Some('2'));
        assert_eq!(row.next(), None);
    }

    #[test]
    fn test_column() {
        let data = "
        12
        34
        ";
        let grid: Grid<char> = Grid::load(data, "");
        let mut row = grid.column(0);
        assert_eq!(row.next(), Some('1'));
        assert_eq!(row.next(), Some('3'));
        assert_eq!(row.next(), None);
    }

    #[test]
    fn test_rows() {
        let data = "
        12
        34
        ";
        let grid: Grid<char> = Grid::load(data, "");
        let mut rows = grid.rows();
        assert_eq!(rows.next().unwrap().collect::<Vec<_>>(), vec!['1', '2']);
        assert_eq!(rows.next().unwrap().collect::<Vec<_>>(), vec!['3', '4']);
        assert!(rows.next().is_none());
    }

    #[test]
    fn test_columns() {
        let data = "
        12
        34
        ";
        let grid: Grid<char> = Grid::load(data, "");
        let mut rows = grid.columns();
        assert_eq!(rows.next().unwrap().collect::<Vec<_>>(), vec!['1', '3']);
        assert_eq!(rows.next().unwrap().collect::<Vec<_>>(), vec!['2', '4']);
        assert!(rows.next().is_none());
    }
}
