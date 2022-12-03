use std::fs;

#[derive(Clone, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn from_encoded_opponent_hand(hand: char) -> Hand {
        match hand {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            _ => panic!("Invalid opponent hand: {}", hand),
        }
    }

    fn from_encoded_player_hand(hand: char) -> Hand {
        match hand {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!("Invalid player hand: {}", hand),
        }
    }

    fn from_wanted_outcome(opponent: &Hand, outcome: &Outcome) -> Hand {
        match outcome {
            Outcome::Win => opponent.beaten_by(),
            Outcome::Lose => opponent.beats(),
            Outcome::Draw => opponent.clone(),
        }
    }

    fn get_score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn beats(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn beaten_by(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_encoded_wanted_outcome(outcome: char) -> Outcome {
        match outcome {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Invalid encoded outcome: {}", outcome),
        }
    }

    fn get_score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

struct Round {
    player: Hand,
    opponent: Hand,
}

impl Round {
    fn get_outcome(&self) -> Outcome {
        if self.player == self.opponent {
            return Outcome::Draw;
        }
        if self.player.beats() == self.opponent {
            return Outcome::Win;
        } else {
            return Outcome::Lose;
        }
    }

    fn get_score(&self) -> i32 {
        let hand_score = self.player.get_score();
        let outcome_score = self.get_outcome().get_score();
        return hand_score + outcome_score;
    }
}

fn main() {
    let input = fs::read_to_string("src/day2/input.txt").unwrap();

    let rounds = input
        .lines()
        .map(|line| Round {
            player: Hand::from_encoded_player_hand(line.chars().nth(2).unwrap()),
            opponent: Hand::from_encoded_opponent_hand(line.chars().nth(0).unwrap()),
        })
        .collect::<Vec<Round>>();
    let sum_score = rounds.iter().map(Round::get_score).sum::<i32>();
    println!("Sum score: {}", sum_score);

    let rounds = input
        .lines()
        .map(|line| {
            let opponent_hand = Hand::from_encoded_opponent_hand(line.chars().nth(0).unwrap());
            let wanted_outcome = Outcome::from_encoded_wanted_outcome(line.chars().nth(2).unwrap());
            let player_hand = Hand::from_wanted_outcome(&opponent_hand, &wanted_outcome);
            Round {
                player: player_hand,
                opponent: opponent_hand,
            }
        })
        .collect::<Vec<Round>>();
    let sum_score = rounds.iter().map(Round::get_score).sum::<i32>();
    println!("Sum score: {}", sum_score);
}
