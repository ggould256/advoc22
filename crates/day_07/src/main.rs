use std::collections::HashMap;
use std::{env};
use std::io::{BufRead, stdin};
use regex::Regex;

fn parse_lines(lines: &Vec<String>) {
    let exit_dir_re = Regex::new(r"\$ cd \.\.").unwrap();
    let enter_dir_re = Regex::new(r"\$ cd (\S+)").unwrap();
    let ls_re = Regex::new(r"\$ ls").unwrap();
    let dir_entry_re = Regex::new(r"dir (\S+)").unwrap();
    let file_entry_re = Regex::new(r"(\d+) (\S+)").unwrap();

    let mut dir_stack: Vec<String> = vec![];
    let mut found_dir_size: HashMap<String, u32> = HashMap::new();

    for line in lines {
        if exit_dir_re.is_match(line) {
            println!("Left dir");
            dir_stack.pop();
        } else if let Some(captures) = enter_dir_re.captures(line) {
            let dir = captures.get(1).unwrap().as_str();
            println!("Entered dir {}", dir);
            dir_stack.push(dir.to_string());
            let new_dirname = dir_stack.join("/");
            found_dir_size.insert(new_dirname, 0);
        } else if ls_re.is_match(line) {
            println!("ls command")
        } else if let Some(captures) = dir_entry_re.captures(line) {
            println!("Found subdirectory {}", captures.get(1).unwrap().as_str());
        } else if let Some(captures) = file_entry_re.captures(line) {
            let filename = captures.get(2).unwrap().as_str();
            let size = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            println!("Found file {} of size {}", filename, size);
            for i in 1..=dir_stack.len() {
                let substack: Vec<String> = dir_stack.iter().take(i).cloned().collect();
                let subdir = substack.join("/");
                *(found_dir_size.get_mut(&subdir).unwrap()) += size;
            }
        } else {
            println!("unrecognized");
        }
    }
    let found_dir_size = found_dir_size;
    let mut small_dir_total = 0;
    for (k, v) in &found_dir_size {
        println!("Directory {} has total size {}", k, v);
        if *v <= 100000 { small_dir_total += *v; }
    }
    println!("Small dir total {}", small_dir_total);
    let total_space = 70000000;
    let used_space = *found_dir_size.get("/").unwrap();
    let free_space = total_space - used_space;
    let required_free_space = 30000000;
    let to_be_deleted = required_free_space - free_space;
    let adequate =
        found_dir_size.iter().filter(|(_, size)| **size >= to_be_deleted);
    let best = adequate.min_by(|(_, lsize), (_, rsize)| lsize.cmp(rsize)).unwrap();
    println!("Best deletion candidate: {} of size {}", best.0, best.1);
}

pub fn main() {
    let _args: Vec<String> = env::args().collect();
    let lines: Vec<String> =
        stdin().lock().lines()
        .map(|x| x.unwrap())
        .collect();
    parse_lines(&lines);
}
