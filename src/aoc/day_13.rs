#[derive(Debug)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}
struct Puzzle {
    games: Vec<Game>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_game = |s: &str| {
            let mut abp = s.lines();
            let (a_x, a_y) = abp.next()?.trim().split_once(": ")?.1.split_once(", ")?;
            let (b_x, b_y) = abp.next()?.trim().split_once(": ")?.1.split_once(", ")?;
            let (p_x, p_y) = abp.next()?.trim().split_once(": ")?.1.split_once(", ")?;
            let button_a = (a_x.trim_start_matches("X+").parse().ok()?, a_y.trim_start_matches("Y+").parse().ok()?);
            let button_b = (b_x.trim_start_matches("X+").parse().ok()?, b_y.trim_start_matches("Y+").parse().ok()?);
            let prize = (p_x.trim_start_matches("X=").parse().ok()?, p_y.trim_start_matches("Y=").parse().ok()?);
            Some(Game {button_a, button_b, prize})
        };
        let games: Option<_> = data.trim().replace("\r", "").split("\n\n").map(parse_game).collect();
        let games = games?;
        Some(Puzzle {games})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn solve_game(game: &Game) -> Option<(i64, i64)> {
    let b = game.button_b.1 * game.button_a.0 - game.button_b.0 * game.button_a.1;
    if b != 0 {
        let b = (game.prize.1 * game.button_a.0 - game.button_a.1 * game.prize.0) / b;
        let a = (game.prize.0 - b * game.button_b.0) / game.button_a.0;
        if ((a * game.button_a.0 + b * game.button_b.0) == game.prize.0) && ((a * game.button_a.1 + b * game.button_b.1) == game.prize.1) {
            Some((a, b))
        } else {
            None
        }
    } else {
        None
    }
}

fn part_1(puzzle: &Puzzle) -> i64 {
    puzzle.games.iter().map(|g| {
        if let Some((a, b)) = solve_game(g) {
            3 * a + b
        } else {
            0
        }
    }).sum()
}

fn part_2(puzzle: &Puzzle) -> i64 {
    puzzle.games.iter().map(|g| {
        let g = Game {prize: (g.prize.0 + 10000000000000, g.prize.1 + 10000000000000), ..*g};
        if let Some((a, b)) = solve_game(&g) {
            3 * a + b
        } else {
            0
        }
    }).sum()
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_13/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_13/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(puzzle.games.len(), 4);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_13/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 480);
    }
}
