struct Puzzle {
    letters: Vec<String>,
    size: (usize, usize),
}

impl Puzzle {
    fn load(data: &str) -> Self {
        let letters: Vec<_> = data.lines().map(|s| s.to_string()).collect();
        let size = (letters[0].len(), letters.len());
        Puzzle {letters, size}
    }

    fn get(&self, r: usize, c: usize) -> char {
        self.letters[r].as_bytes()[c] as char
    }
}

struct PuzzleIterator<'a> {
    data: &'a Puzzle,
    position: usize,
    direction: usize
}

impl<'a> PuzzleIterator<'a> {
    fn next_direction(&mut self) {
        self.position = 0;
        self.direction += 1;
    }

    fn next_position(&mut self) {
        self.position += 1;
        match self.direction {
            0 => {
                if self.position == self.data.size.1 {
                    self.next_direction();
                }
            }
            1 => {
                if self.position == self.data.size.0 {
                    self.next_direction();
                }
            }
            2..=3 => {
                if self.position == ((self.data.size.0 + self.data.size.1) - 1) {
                    self.next_direction();
                }
            }
            _ => ()
        }
    }
}

impl<'a> IntoIterator for &'a Puzzle {
    type Item = String;
    type IntoIter = PuzzleIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PuzzleIterator {data: self, position: 0, direction: 0}
    }
}

impl<'a> Iterator for PuzzleIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            0 => {
                let r = self.data.letters[self.position].clone();
                self.next_position();
                Some(r)
            }
            1 => {
                let mut r = String::new();
                for s in &self.data.letters {
                    r.push(s.as_bytes()[self.position] as char);
                }
                self.next_position();
                Some(r)
            }
            2 => {
                let mut r = String::new();
                let br; let bc;
                if self.position < self.data.size.1 {
                    br = self.position;
                    bc = 0;
                } else {
                    br = self.data.size.1 - 1;
                    bc = self.position - self.data.size.1 + 1;
                }
                let s = std::cmp::min(br + 1, self.data.size.0 - bc);
                for i in 0..s {
                    r.push(self.data.letters[br - i].as_bytes()[bc + i] as char);
                }
                self.next_position();
                Some(r)
            }
            3 => {
                let mut r = String::new();
                let (br, bc) = if self.position < self.data.size.1 {
                    ((self.data.size.1 - 1) - self.position, 0)
                } else {
                    (0, self.position - self.data.size.1 + 1)
                };
                let s = std::cmp::min(self.data.size.1 - br, self.data.size.0 - bc);
                for i in 0..s {
                    r.push(self.data.letters[br + i].as_bytes()[bc + i] as char);
                }
                self.next_position();
                Some(r)
            }
            _ => None
        }
    }
}

fn part_1(puzzle: &Puzzle) -> u32 {
    let mut total = 0;
    for s in puzzle {
        let nf = s.match_indices("XMAS").count();
        let nb = s.match_indices("SAMX").count();
        total += nf + nb;
    }
    total as u32
}

fn part_2(puzzle: &Puzzle) -> u32 {
    let bc = 1;
    let ec = puzzle.size.0 - 1;
    let br = 1;
    let er = puzzle.size.1 - 1;
    let mut total = 0;

    for r in br..er {
        for c in bc..ec {
            let mrmc = puzzle.get(r - 1, c - 1);
            let mrpc = puzzle.get(r - 1, c + 1);
            let prmc = puzzle.get(r + 1, c - 1);
            let prpc = puzzle.get(r + 1, c + 1);
            if puzzle.get(r, c) == 'A'
                && ((mrmc == 'M' && prpc == 'S') || (mrmc == 'S' && prpc == 'M'))
                && ((mrpc == 'M' && prmc == 'S') || (mrpc == 'S' && prmc == 'M')) {
                total += 1;
            }
        }
    }
    total
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_04/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = include_str!("../../data/day_04/test.txt");

    #[test]
    fn test_data() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(puzzle.size.1, 10);
        assert_eq!(puzzle.letters.len(), puzzle.size.1);
        assert_eq!(puzzle.size.0, 10);
        assert!(puzzle.letters.iter().all(|line| line.len() == puzzle.size.0));
    }

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(part_1(&puzzle), 18);
    }

    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(part_2(&puzzle), 9);
    }
}
