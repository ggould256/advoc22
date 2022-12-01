use std::io::{self, BufRead};
use std::cmp::{min, max};

pub fn main() {
    let mut elf_rations: Vec<Vec<u32>> = vec![vec![0]];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_text = line.unwrap();
        if line_text.is_empty() {
            elf_rations.push(vec![]);
        } else {
            let ration_cals: u32 = line_text.parse().unwrap();
            elf_rations.last_mut().expect("no items in list")
                .push(ration_cals);
        }
    }
    let mut elf_cals: Vec<u32> = vec![0];
    for elf in elf_rations {
        let mut total_cals: u32 = 0;
        for ration in elf {
            total_cals += ration;
        }
        elf_cals.push(total_cals);
    }
    elf_cals.sort();
    let num_to_print = max(0, min(elf_cals.len(), 3));
    let mut top_n_carried_cals = 0;
    for i in 0..num_to_print {
        top_n_carried_cals += elf_cals[elf_cals.len() - i - 1];
    }
    println!("{}", top_n_carried_cals);
}
