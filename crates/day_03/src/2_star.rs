use std::env;
use std::io::{BufRead, stdin};
use std::collections::HashSet;

fn char_to_priority(c: char) -> u8 {
    if c.is_lowercase() { (c as u8) + 1 - b'a' }
    else { (c as u8) + 26 + 1 - b'A' }
}

#[allow(dead_code)]
fn priority_to_char(i: u8) -> char {
    if i <= 26 { (i + b'a' - 1) as char }
    else { (i - 26 + b'A' - 1) as char }
}

fn only(s: &HashSet<u8>) -> u8 { assert!(s.len() == 1); *s.iter().next().unwrap() }

fn sack_to_set(sack: &str) -> HashSet<u8> {
    HashSet::from_iter(
        sack.chars().map(char_to_priority))
}

fn overlap_prio(lines: &[String]) -> u8 {
    assert!(!lines.is_empty());
    let mut overlap = sack_to_set(&lines[0]);
    for line in lines.iter().skip(1) {
        overlap = HashSet::from_iter(overlap.intersection(&sack_to_set(line)).copied());
    }
    only(&overlap)
}

pub fn main() {
    let _args: Vec<String> = env::args().collect();
    let lines: Vec<String> = stdin().lock()
        .lines().map(|x| x.unwrap())
        .filter(|s| !s.is_empty())
        .collect();
    let mut prio_sum: u16 = 0;
    for group_num in 0..(lines.len() / 3) {
        let group_start = group_num * 3;
        let group = &lines[group_start..group_start+3];
        prio_sum += overlap_prio(group) as u16;
    }
    println!("{}", prio_sum);
}
