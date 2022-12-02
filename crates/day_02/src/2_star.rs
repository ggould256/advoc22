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

fn play_from_int(i: i32) -> Play {
    println!("play_from_int({})", i);
    let discr = (i + 2) % 3 + 1;
    match discr {
        1 => Play::Rock, 2 => Play::Paper, 3 => Play::Scissors,
        _ => todo!()
    }
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
    println!("I play {:?}, they play {:?}, score {} + {} = {}",
             (my_play as i32), (opponent_play as i32), play_score, result_score,
             play_score + result_score);
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
                'X' => my_play = play_from_int(opponent_play as i32 - 1),
                'Y' => my_play = opponent_play,
                'Z' => my_play = play_from_int(opponent_play as i32 + 1),
                _ => (),
            }
        }
        assert!(opponent_play != Play::Invalid);
        assert!(my_play != Play::Invalid);
        score += score_round(opponent_play, my_play);
    }
    println!("{}", score);
}
