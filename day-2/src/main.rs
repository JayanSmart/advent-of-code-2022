use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

use common::read_lines;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Play {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Play {
    type Err = String;

    fn from_str(input: &str) -> Result<Play, Self::Err> {
        match input {
            // Opponent
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissor),
            _ => Err("Unable to Match".parse().unwrap()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            // Opponent
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("Unable to Match".parse().unwrap()),
        }
    }
}

struct Strategy {
    desired_outcome: Outcome,
    opponent: Play,
    to_play: Play,
}

impl Strategy {
    pub fn new(desired_outcome: Outcome, opponent: Play) -> Self {
        Self {
            desired_outcome,
            opponent,
            to_play: self::Strategy::what_to_play(desired_outcome, opponent),
        }
    }

    fn what_to_play(desired_outcome: Outcome, opponent: Play) -> Play {
        /// Return the Play required to achieve the desired result
        match desired_outcome {
            Outcome::Win => match opponent {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissor,
                Play::Scissor => Play::Rock,
            },
            Outcome::Draw => match opponent {
                Play::Rock => Play::Rock,
                Play::Paper => Play::Paper,
                Play::Scissor => Play::Scissor,
            },
            Outcome::Loss => match opponent {
                Play::Rock => Play::Scissor,
                Play::Paper => Play::Rock,
                Play::Scissor => Play::Paper,
            },
        }
    }
}

fn calculate_result(you: &Play, opponent: &Play) -> Outcome {
    if you == opponent {
        return Outcome::Draw;
    }

    match you {
        Play::Rock => {
            if opponent == &Play::Paper {
                return Outcome::Loss;
            }
        }
        Play::Paper => {
            if opponent == &Play::Scissor {
                return Outcome::Loss;
            }
        }
        Play::Scissor => {
            if opponent == &Play::Rock {
                return Outcome::Loss;
            }
        }
    }

    Outcome::Win
}

fn score_for_played(played: &Play) -> i32 {
    match played {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissor => 3,
    }
}

fn score_for_outcome(result: Outcome) -> i32 {
    match result {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn score_for_match(you: Play, opponent: Play) -> i32 {
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
                        Outcome::from_str(s.last().unwrap()).unwrap(),
                        Play::from_str(s.first().unwrap()).unwrap(),
                    );

                    let score = score_for_match(strat.to_play, strat.opponent);
                    total_score += score;

                    dbg!(
                        count,
                        strat.opponent,
                        strat.desired_outcome,
                        strat.to_play,
                        score,
                        total_score
                    );
                }
                None => {}
            }
            count += 1;
        }

        println!("Total score: {:?}", total_score);
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_basic_game_rules() {
        //Rock
        assert_eq!(calculate_result(&Play::Rock, &Play::Scissor), Outcome::Win);
        assert_eq!(calculate_result(&Play::Rock, &Play::Rock), Outcome::Draw);
        assert_eq!(calculate_result(&Play::Rock, &Play::Paper), Outcome::Loss);

        //Paper
        assert_eq!(calculate_result(&Play::Paper, &Play::Rock), Outcome::Win);
        assert_eq!(calculate_result(&Play::Paper, &Play::Paper), Outcome::Draw);
        assert_eq!(
            calculate_result(&Play::Paper, &Play::Scissor),
            Outcome::Loss
        );

        //Scissor
        assert_eq!(calculate_result(&Play::Scissor, &Play::Paper), Outcome::Win);
        assert_eq!(
            calculate_result(&Play::Scissor, &Play::Scissor),
            Outcome::Draw
        );
        assert_eq!(calculate_result(&Play::Scissor, &Play::Rock), Outcome::Loss);
    }

    #[test]
    fn test_match_score() {
        //Rock (1 point + outcome)
        assert_eq!(score_for_match(Play::Rock, Play::Scissor), 1 + 6);
        assert_eq!(score_for_match(Play::Rock, Play::Rock), 1 + 3);
        assert_eq!(score_for_match(Play::Rock, Play::Paper), 1);

        //Paper (2 points + outcome)
        assert_eq!(score_for_match(Play::Paper, Play::Rock), 2 + 6);
        assert_eq!(score_for_match(Play::Paper, Play::Paper), 2 + 3);
        assert_eq!(score_for_match(Play::Paper, Play::Scissor), 2);

        //Scissor (3 points + outcome)
        assert_eq!(score_for_match(Play::Scissor, Play::Paper), 3 + 6);
        assert_eq!(score_for_match(Play::Scissor, Play::Scissor), 3 + 3);
        assert_eq!(score_for_match(Play::Scissor, Play::Rock), 3);
    }

    #[test]
    fn test_to_play_calculations() {
        // To Win
        assert_eq!(
            Strategy::what_to_play(Outcome::Win, Play::Rock),
            Play::Paper
        );
        assert_eq!(
            Strategy::what_to_play(Outcome::Win, Play::Paper),
            Play::Scissor
        );
        assert_eq!(
            Strategy::what_to_play(Outcome::Win, Play::Scissor),
            Play::Rock
        );

        // To Draw
        assert_eq!(
            Strategy::what_to_play(Outcome::Draw, Play::Rock),
            Play::Rock
        );
        assert_eq!(
            Strategy::what_to_play(Outcome::Draw, Play::Paper),
            Play::Paper
        );
        assert_eq!(
            Strategy::what_to_play(Outcome::Draw, Play::Scissor),
            Play::Scissor
        );

        // To Loose
        assert_eq!(
            Strategy::what_to_play(Outcome::Loss, Play::Rock),
            Play::Scissor
        );
        assert_eq!(
            Strategy::what_to_play(Outcome::Loss, Play::Paper),
            Play::Rock
        );
        assert_eq!(
            Strategy::what_to_play(Outcome::Loss, Play::Scissor),
            Play::Paper
        );
    }
}
