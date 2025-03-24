/*
numeric keypad

+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

directional keypad

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

*/

struct NumPad {
    position: (usize, usize)
}

impl NumPad {
    fn get_position(key: char) -> (usize, usize) {
        match key {
            'A' => (2, 3),
            '0' => (1, 3),
            '1' => (0, 2),
            '2' => (1, 2),
            '3' => (2, 2),
            '4' => (0, 1),
            '5' => (1, 1),
            '6' => (2, 1),
            '7' => (0, 0),
            '8' => (1, 0),
            '9' => (2, 0),
            _ => panic!("invalid key")
        }
    }
}

struct DirPad {
    position: (usize, usize)
}

impl DirPad {
    fn get_position(key: char) -> (usize, usize)  {
        match key {
            'A' => (2, 0),
            '^' => (1, 0),
            'v' => (2, 1),
            '<' => (0, 1),
            '>' => (1, 1),
            _ => panic!("invalid key")
        }
    }
}

struct Puzzle;

fn part_1(puzzle: &Puzzle) -> usize {
    todo!("part 1");
}

fn part_2(puzzle: &Puzzle) -> usize {
    todo!("part 2");
}

pub(crate) fn solve() {
    // let puzzle = Puzzle::load();
    let puzzle = Puzzle {};
    part_1(&puzzle);
    part_2(&puzzle);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
    }

    #[test]
    fn test_part_2() {
    }
}