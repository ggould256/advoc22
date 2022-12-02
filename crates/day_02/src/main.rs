use std::env;
use std::io::{BufRead, stdin};
use std::cmp::{min, max};

pub fn stanzas(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut results : Vec<Vec<String>> = vec![vec![]];
    for line_text in lines {
        if line_text.is_empty() {
            results.push(vec![]);
        } else {
            results.last_mut().unwrap().push(line_text.clone());
        }
    }
    results
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let n_elves: usize = {
        if args.len() <= 1 { 1 } else {
            args[1].parse().expect("expected int")}};
    let lines = stdin().lock().lines().map(|x| x.unwrap());
    let stanzas = stanzas(lines.collect());
    let rations = stanzas.iter().map(
        |s| s.iter().map(
            |l| l.parse().unwrap()).collect());
    let mut ration_totals: Vec<u32> = rations.map(
        |v: Vec<u32>| v.iter().sum()).collect();

    ration_totals.sort();
    let ration_totals = ration_totals;
    let num_to_print = max(0, min(ration_totals.len(), n_elves));
    let result: u32 = ration_totals[ration_totals.len() - num_to_print ..].iter().sum();
    println!("{}", result);
}
