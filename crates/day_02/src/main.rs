use std::env;
use std::io::{BufRead, stdin};

#[derive(PartialEq, Clone)]
#[derive(Copy)]
enum Play {
    Invalid = -99,
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn score_round(opponent_play: Play, my_play: Play) -> i32 {
    let play_score: i32 = my_play as i32;
    let result = (my_play as i32) - (opponent_play as i32) + 3;
    let result_score = match result % 3 {
        0 => 3,
        1 => 6,
        2 => 0,
        _ => todo!()
    };
    play_score + result_score
}

pub fn main() {
    let _args: Vec<String> = env::args().collect();
    let lines = stdin().lock().lines().map(|x| x.unwrap());
    let mut score = 0;
    for line in lines {
        if line.is_empty() { continue; }
        let mut opponent_play = Play::Invalid;
        let mut my_play = Play::Invalid;
        for c in line.chars() {
            match c {
                'A' => opponent_play = Play::Rock,
                'B' => opponent_play = Play::Paper,
                'C' => opponent_play = Play::Scissors,
                'X' => my_play = Play::Rock,
                'Y' => my_play = Play::Paper,
                'Z' => my_play = Play::Scissors,
                _ => (),
            }
        }
        assert!(opponent_play != Play::Invalid);
        assert!(my_play != Play::Invalid);
        score += score_round(opponent_play, my_play);
    }
    println!("{}", score);
}
