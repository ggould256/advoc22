use std::env;
use std::io::{BufRead, stdin};

pub fn main() {
    let _args: Vec<String> = env::args().collect();
    let lines = stdin().lock().lines().map(|x| x.unwrap());
    let re = regex::Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut enclosures = 0;
    let mut overlaps = 0;
    for line in lines {
        if line.is_empty() {continue;}
        assert!(re.is_match(&line));
        let captures = re.captures(&line).unwrap();
        assert_eq!(captures.len(), 5);
        let terms: Vec<u32> = captures
            .iter().skip(1)
            .map(|m| m.expect("Could not parse string").as_str()
                                     .parse().expect("Not an integer"))
            .collect();
        if let [ll, lu, rl, ru] = terms[..] {
            println!("{}-{},{}-{}", ll, lu, rl, ru);
            if (ll <= rl) && (lu >= ru) {
                println!("ENCLOSURE! (R in L)");
                enclosures += 1;
                overlaps += 1;
            } else if (ll >= rl) && (lu <= ru) {
                println!("ENCLOSURE! (L in R)");
                enclosures += 1;
                overlaps += 1;
            } else if ((ll <= rl) && (lu >= rl)) || (rl <= ll) && (ru >= ll) {
                println!("OVERLAP!");
                overlaps += 1;
            }
        } else {
            println!("parse error");
        }
    }
    println!("{} enclosures", enclosures);
    println!("{} overlaps", overlaps);
}
