use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;
use std::{env, io};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Plays {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Plays {
    type Err = String;

    fn from_str(input: &str) -> Result<Plays, Self::Err> {
        match input {
            // Opponent
            "A" => Ok(Plays::Rock),
            "B" => Ok(Plays::Paper),
            "C" => Ok(Plays::Scissor),

            // You
            "X" => Ok(Plays::Rock),
            "Y" => Ok(Plays::Paper),
            "Z" => Ok(Plays::Scissor),
            _ => Err("Unable to Match".parse().unwrap()),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

struct Strategy {
    you: Plays,
    opponent: Plays,
}

impl Strategy {
    pub fn new(you: Plays, opponent: Plays) -> Self {
        Self { you, opponent }
    }
}

fn calculate_result(you: &Plays, opponent: &Plays) -> Outcome {
    if you == opponent {
        return Outcome::Draw;
    }

    match you {
        Plays::Rock => {
            if opponent == &Plays::Paper {
                return Outcome::Loss;
            }
        }
        Plays::Paper => {
            if opponent == &Plays::Scissor {
                return Outcome::Loss;
            }
        }
        Plays::Scissor => {
            if opponent == &Plays::Rock {
                return Outcome::Loss;
            }
        }
    }

    Outcome::Win
}

fn score_for_played(played: &Plays) -> i32 {
    match played {
        Plays::Rock => 1,
        Plays::Paper => 2,
        Plays::Scissor => 3,
    }
}

fn score_for_outcome(result: Outcome) -> i32 {
    match result {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn score_for_match(you: Plays, opponent: Plays) -> i32 {
    score_for_played(&you) + score_for_outcome(calculate_result(&you, &opponent))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut total_score: i32 = 0;

    println!("In file {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        let mut count = 0;
        for line in lines {
            match line.ok() {
                Some(strategy) => {
                    let s = strategy.split_whitespace().collect::<Vec<&str>>();
                    let strat = Strategy::new(
                        /// We have to swap last and first here as I coded everything else to be
                        /// you, opponent and the input is swapped. If I do this again, swap
                        /// everything to match input order.
                        Plays::from_str(s.last().unwrap()).unwrap(),
                        Plays::from_str(s.first().unwrap()).unwrap(),
                    );

                    let score = score_for_match(strat.you, strat.opponent);
                    total_score += score;

                    dbg!(count, strat.you, strat.opponent, score, total_score);
                }
                None => {}
            }
            count += 1;
        }

        println!("Total score: {:?}", total_score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic_game_rules() {
        //Rock
        assert_eq!(
            calculate_result(&Plays::Rock, &Plays::Scissor),
            Outcome::Win
        );
        assert_eq!(calculate_result(&Plays::Rock, &Plays::Rock), Outcome::Draw);
        assert_eq!(calculate_result(&Plays::Rock, &Plays::Paper), Outcome::Loss);

        //Paper
        assert_eq!(calculate_result(&Plays::Paper, &Plays::Rock), Outcome::Win);
        assert_eq!(
            calculate_result(&Plays::Paper, &Plays::Paper),
            Outcome::Draw
        );
        assert_eq!(
            calculate_result(&Plays::Paper, &Plays::Scissor),
            Outcome::Loss
        );

        //Scissor
        assert_eq!(
            calculate_result(&Plays::Scissor, &Plays::Paper),
            Outcome::Win
        );
        assert_eq!(
            calculate_result(&Plays::Scissor, &Plays::Scissor),
            Outcome::Draw
        );
        assert_eq!(
            calculate_result(&Plays::Scissor, &Plays::Rock),
            Outcome::Loss
        );
    }

    #[test]
    fn test_match_score() {
        //Rock (1 point + outcome)
        assert_eq!(score_for_match(Plays::Rock, Plays::Scissor), 1 + 6);
        assert_eq!(score_for_match(Plays::Rock, Plays::Rock), 1 + 3);
        assert_eq!(score_for_match(Plays::Rock, Plays::Paper), 1);

        //Paper (2 points + outcome)
        assert_eq!(score_for_match(Plays::Paper, Plays::Rock), 2 + 6);
        assert_eq!(score_for_match(Plays::Paper, Plays::Paper), 2 + 3);
        assert_eq!(score_for_match(Plays::Paper, Plays::Scissor), 2);

        //Scissor (3 points + outcome)
        assert_eq!(score_for_match(Plays::Scissor, Plays::Paper), 3 + 6);
        assert_eq!(score_for_match(Plays::Scissor, Plays::Scissor), 3 + 3);
        assert_eq!(score_for_match(Plays::Scissor, Plays::Rock), 3);
    }
}
