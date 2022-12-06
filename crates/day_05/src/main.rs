use std::env;
use std::io::{BufRead, stdin};

type Layout = Vec<Vec<char>>;

fn read_layout(lines: &[String]) -> Layout {
    let mut layout = vec![];
    let mut lines_vec = lines.to_vec();
    lines_vec.reverse();
    for line in lines_vec.iter().skip(1) {
        let num_columns = (line.len() + 1) / 4;
        for col_num in 0..num_columns {
            let start_point = col_num * 4;
            let box_str: &str = &line[start_point..(start_point+3)];
            println!("Saw box {}", box_str);
        }
    }
    vec![]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Move {
    count: u8,
    from: u8,
    to: u8
}

fn read_moves(lines: &[String]) -> Vec<Move> {
    let move_re = regex::Regex::new(
        r"move (\d+) from (\d+) to (\d+)"
      ).expect("Regex parse error");
    vec![]
}

fn perform_move(m: Move, layout: &mut Layout) {

}

fn read_top_crates(layout: &Layout) -> String {
    "".to_string()
}

pub fn main() {
    let _args: Vec<String> = env::args().collect();
    let lines: Vec<String> =
        stdin().lock().lines()
        .map(|x| x.unwrap())
        .collect();
    let parts: Vec<&[String]> = lines.split(String::is_empty).collect();
    let layout_text: &[String] = parts[0];
    let moves_text: &[String] = parts[1];

    println!("Parsed {} lines of layout and {} lines of moves",
             layout_text.len(), moves_text.len());

    let initial_layout = read_layout(&layout_text[..]);
    let moves = read_moves(&moves_text[..]);
    let mut layout = initial_layout;
    for m in moves {
        perform_move(m, &mut layout);
    }
    println!("Top crates: {}", read_top_crates(&layout));
}
