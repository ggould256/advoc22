use std::env;
use std::io::{BufRead, stdin};
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let num_unique_chars: usize = args[1].parse().expect("Not an integer");
    let lines = stdin().lock().lines().map(|x| x.unwrap());
    for line in lines {
        let mut buffer: VecDeque<char> = VecDeque::new();
        let mut counter: usize = 0;
        for c in line.chars() {
            counter += 1;
            buffer.push_back(c);
            while buffer.len() > num_unique_chars { buffer.pop_front(); }
            if buffer.len() == num_unique_chars {
                let unique_chars: HashSet<char> = HashSet::from_iter(buffer.iter().copied());
                if unique_chars.len() == num_unique_chars {
                    println!("Sequence found ending at {}", counter);
                    break;
                }
            }
        }
    }
}
