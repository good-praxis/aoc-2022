use common::*;

fn main() {
    let mut score: u32 = 0;
    let mut real_score: u32 = 0;

    #[derive(PartialEq, Clone)]
    enum RPS {
        Rock,
        Paper,
        Scissors,
    }
    enum GameState {
        Win,
        Draw,
        Loss,
    }
    use GameState::*;
    use RPS::*;

    fn determine_move(char: char) -> RPS {
        match char {
            'A' | 'X' => Some(Rock),
            'B' | 'Y' => Some(Paper),
            'C' | 'Z' => Some(Scissors),
            _ => None,
        }
        .unwrap()
    }

    fn determine_player_move(enemy: &RPS, char: char) -> RPS {
        match char {
            'X' => Some(get_loss(enemy)),
            'Y' => Some(enemy.clone()),
            'Z' => Some(get_win(enemy)),
            _ => None,
        }
        .unwrap()
    }

    fn get_win(against: &RPS) -> RPS {
        match against {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
    fn get_loss(against: &RPS) -> RPS {
        get_win(&get_win(against))
    }

    fn am_i_winning_dad(enemy: &RPS, player: &RPS) -> GameState {
        match (enemy, player) {
            (x, y) if x == y => Draw,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Win,
            _ => Loss,
        }
    }

    fn calc_score(enemy: &RPS, player: &RPS) -> u32 {
        let score = match am_i_winning_dad(enemy, player) {
            Win => 6,
            Draw => 3,
            Loss => 0,
        };
        score
            + match player {
                Rock => 1,
                Paper => 2,
                Scissors => 3,
            }
    }

    for line in get_lines_from_file("day-02") {
        let chars: Vec<char> = line.unwrap().chars().collect();
        let (enemy, player) = (chars[0], chars[2]);
        let (enemy, player) = (determine_move(enemy), determine_move(player));
        score += calc_score(&enemy, &player);
        real_score += calc_score(&enemy, &determine_player_move(&enemy, chars[2]))
    }

    present_result(score, Some(real_score));
}
