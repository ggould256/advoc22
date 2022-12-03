use std::env;
use std::io::{BufRead, stdin};
use std::collections::HashSet;

fn char_to_priority(c: char) -> u8 {
    if c.is_lowercase() { (c as u8) + 1 - b'a' }
    else { (c as u8) + 26 + 1 - b'A' }
}

fn priority_to_char(i: u8) -> char {
    if i <= 26 { (i + b'a' - 1) as char }
    else { (i - 26 + b'A' - 1) as char }
}

fn only(s: &HashSet<u8>) -> u8 { assert!(s.len() == 1); *s.iter().next().unwrap() }

fn overlap_prio(line: &str) -> u8 {
    let num_items = line.len();
    assert!(num_items % 2 == 0);
    println!("{}", line);
    let items_per_sack = num_items / 2;
    let left: HashSet<u8> = HashSet::from_iter(
        line.chars().take(items_per_sack).map(char_to_priority));
    let right: HashSet<u8> = HashSet::from_iter(
        line.chars().skip(items_per_sack).map(char_to_priority));
    let overlap: HashSet<u8> = HashSet::from_iter(left.intersection(&right).copied());
    println!("Left is {:?} right is {:?}", left, right);
    println!("Overlap is {:?}", overlap);
    assert!(overlap.len() == 1);
    println!("Overlap is {}", priority_to_char(only(&overlap)));
    only(&overlap)
}

pub fn main() {
    let _args: Vec<String> = env::args().collect();
    let lines = stdin().lock().lines().map(|x| x.unwrap());
    let mut prio_sum: u16 = 0;
    for line in lines {
        if line.is_empty() { continue; }
        prio_sum += overlap_prio(&line) as u16;
    }
    println!("{}", prio_sum);
}
