use std::env;
use std::io::{BufRead, stdin};

type Layout = Vec<Vec<char>>;

fn read_layout(lines: &[String]) -> Layout {
    let mut layout: Layout = vec![];
    let mut lines_vec = lines.to_vec();
    lines_vec.reverse();
    for line in lines_vec.iter().skip(1) {
        let num_columns = (line.len() + 1) / 4;
        for col_num in 0..num_columns {
            if layout.len() < col_num + 1 {
                layout.push(vec![]);
            }
            let start_point = col_num * 4;
            let box_str: &str = &line[start_point..(start_point+3)];
            let box_name: char =
                box_str.chars().nth(1).expect("couldn't get box name");
            if box_name != ' ' {
                layout[col_num].push(box_name);
            }
        }
    }
    layout
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize
}

fn read_moves(lines: &[String]) -> Vec<Move> {
    let move_re = regex::Regex::new(
        r"move (\d+) from (\d+) to (\d+)"
      ).expect("Regex parse error");
    let mut result: Vec<Move> = vec![];
    for line in lines {
        println!("Parsing line {}", line);
        let captures = move_re.captures(line).expect("Not a valid move");
        let this_move: Move = Move{
            count: captures.get(1).expect("not enough matches")
                           .as_str().parse().expect("not a valid number"),
            from: captures.get(2).expect("not enough matches")
                          .as_str().parse().expect("not a valid number"),
            to: captures.get(3).expect("not enough matches")
                        .as_str().parse().expect("not a valid number"),
        };
        println!("Move({}, {}, {})", this_move.count, this_move.from, this_move.to);
        result.push(this_move);
    }
    result
}

fn perform_multi_move(m: Move, layout: &mut Layout) {
    for _i in 0..m.count {
        let payload = layout[m.from - 1].pop().expect("No box available in column");
        layout[m.to - 1].push(payload);
        println!("Moved {} from {} to {}", payload, m.from, m.to);
    }
}

fn perform_stack_move(m: Move, layout: &mut Layout) {
    let origin_stack_len = layout[m.from - 1].len();
    let mut payload = vec![];
    payload.extend_from_slice(&layout[m.from - 1][origin_stack_len-m.count..]);
    layout[m.to - 1].extend_from_slice(&payload);
    layout[(m.from - 1)].truncate(origin_stack_len-m.count);
}

fn read_top_crates(layout: &Layout) -> String {
    let mut result: String = "".to_string();
    for col in layout {
        result += &col.last().unwrap_or(&'*').to_string();
    }
    result
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let lines: Vec<String> =
        stdin().lock().lines()
        .map(|x| x.unwrap())
        .collect();
    let parts: Vec<&[String]> = lines.split(String::is_empty).collect();
    let layout_text: &[String] = parts[0];
    let moves_text: &[String] = parts[1];

    println!("Parsed {} lines of layout and {} lines of moves",
             layout_text.len(), moves_text.len());

    let initial_layout = read_layout(layout_text);
    let moves = read_moves(moves_text);
    let mut layout = initial_layout;
    for m in moves {
        if args[1] == "stacked" {
            perform_stack_move(m, &mut layout);
        } else if args[1] == "repeated" {
            perform_multi_move(m, &mut layout);
        } else {
            todo!("not implemented")
        }
        println!("{}", read_top_crates(&layout));
    }
    println!("Top crates: {}", read_top_crates(&layout));
}
