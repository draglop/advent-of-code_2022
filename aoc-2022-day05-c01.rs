// https://adventofcode.com/2022/day/5
// (part 1)

const CRATE_STR_LEN: usize = 4; // include separator
const CRATE_STR_CRATE_INDEX: usize = 1; // index of the crate in the str
const COMMAND_MOVE_PREFIX: &str = "move ";
const COMMAND_FROM_PREFIX: &str = " from ";
const COMMAND_TO_PREFIX: &str = " to ";

fn stacks_build(lines: &mut Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let lines_count: usize = lines.len();
    if lines_count > 0 {
        let crates_line_len: usize = lines[0].len();
        // create stacks
        let stacks_count: usize = (crates_line_len / CRATE_STR_LEN) + 1; // +1 cause the last crate doesn't have a separator
        for __ in 0..stacks_count {
            stacks.push(Vec::new());
        }
        // populate stacks
        let mut lines_i: usize = 0;
        while lines_i < lines_count {
            let crate_line = lines[lines_i];
            if crate_line.len() == 0 {
                // end of crates
                break;
            }
            lines_i += 1;
            for i in 0..stacks_count {
                let c: char = crate_line
                    .chars()
                    .nth(i * CRATE_STR_LEN + CRATE_STR_CRATE_INDEX)
                    .unwrap();
                if c != ' ' {
                    stacks[i].insert(0, c);
                }
            }
        }
        // strip crate lines from input
        lines.drain(0..lines_i + 1);

        // remove last crate of each stacks, it's the stacks index
        for stack in &mut stacks {
            stack.remove(0);
        }
    }

    return stacks;
}

fn number_parse(number: &str) -> (usize, usize) {
    let mut digit_count: usize = 0;
    for c in number.chars() {
        if c < '0' || c > '9' {
            break;
        }
        digit_count += 1;
    }
    let n = number
        .get(0..digit_count)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    return (n, digit_count);
}

fn command_parse(command: &str) -> (usize, usize, usize) {
    // move count
    assert_eq!(
        Some(COMMAND_MOVE_PREFIX),
        command.get(0..COMMAND_MOVE_PREFIX.len())
    );
    let (cmd_move_count, move_char_read_count) =
        number_parse(command.get(COMMAND_MOVE_PREFIX.len()..).unwrap());
    // from index
    let from_str_index = COMMAND_MOVE_PREFIX.len() + move_char_read_count;
    assert_eq!(
        Some(COMMAND_FROM_PREFIX),
        command.get(from_str_index..from_str_index + COMMAND_FROM_PREFIX.len())
    );
    let (cmd_from_index, from_char_read_count) = number_parse(
        command
            .get(from_str_index + COMMAND_FROM_PREFIX.len()..)
            .unwrap(),
    );
    // to index
    let to_str_index = from_str_index + COMMAND_FROM_PREFIX.len() + from_char_read_count;
    assert_eq!(
        Some(COMMAND_TO_PREFIX),
        command.get(to_str_index..to_str_index + COMMAND_TO_PREFIX.len())
    );
    let (cmd_to_index, to_char_read_count) = number_parse(
        command
            .get(to_str_index + COMMAND_TO_PREFIX.len()..)
            .unwrap(),
    );
    assert!(
        command.len()
            == COMMAND_MOVE_PREFIX.len()
                + move_char_read_count
                + COMMAND_FROM_PREFIX.len()
                + from_char_read_count
                + COMMAND_TO_PREFIX.len()
                + to_char_read_count
    );

    return (cmd_move_count, cmd_from_index, cmd_to_index);
}

fn crates_move(stacks: &mut Vec<Vec<char>>, lines: &Vec<&str>) {
    for line in lines {
        let (count, from, to) = command_parse(line);
        // indices from the command starts at 1
        assert!(from > 0);
        assert!(to > 0);
        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    }
}

fn crates_rearrange(list: &str) -> String {
    let mut lines: std::vec::Vec<&str> = list.split('\n').collect();

    let mut stacks: Vec<Vec<char>> = stacks_build(&mut lines);
    crates_move(&mut stacks, &lines);

    let mut s: String = String::with_capacity(stacks.len());
    for stack in stacks {
        s.push(stack[stack.len() - 1]);
    }

    return s;
}

fn main() {
    // example
    assert_eq!("CMZ", crates_rearrange("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2"));
    //// user puzzle input
    assert_eq!("FRDSQRRCD", crates_rearrange("        [M]     [B]             [N]\n[T]     [H]     [V] [Q]         [H]\n[Q]     [N]     [H] [W] [T]     [Q]\n[V]     [P] [F] [Q] [P] [C]     [R]\n[C]     [D] [T] [N] [N] [L] [S] [J]\n[D] [V] [W] [R] [M] [G] [R] [N] [D]\n[S] [F] [Q] [Q] [F] [F] [F] [Z] [S]\n[N] [M] [F] [D] [R] [C] [W] [T] [M]\n 1   2   3   4   5   6   7   8   9 \n\nmove 1 from 8 to 7\nmove 1 from 2 to 7\nmove 6 from 9 to 8\nmove 1 from 9 to 1\nmove 1 from 9 to 1\nmove 3 from 3 to 6\nmove 3 from 3 to 9\nmove 1 from 9 to 2\nmove 5 from 7 to 9\nmove 9 from 1 to 6\nmove 3 from 4 to 9\nmove 2 from 9 to 2\nmove 1 from 4 to 2\nmove 1 from 3 to 9\nmove 8 from 9 to 4\nmove 14 from 6 to 7\nmove 1 from 3 to 2\nmove 5 from 4 to 2\nmove 5 from 5 to 7\nmove 4 from 2 to 1\nmove 2 from 4 to 9\nmove 1 from 4 to 3\nmove 3 from 5 to 7\nmove 1 from 8 to 6\nmove 2 from 8 to 7\nmove 2 from 1 to 2\nmove 1 from 9 to 7\nmove 2 from 1 to 3\nmove 5 from 6 to 5\nmove 4 from 5 to 7\nmove 3 from 8 to 4\nmove 20 from 7 to 1\nmove 11 from 7 to 5\nmove 1 from 6 to 9\nmove 3 from 9 to 2\nmove 12 from 1 to 9\nmove 2 from 8 to 3\nmove 4 from 2 to 8\nmove 8 from 2 to 1\nmove 4 from 8 to 9\nmove 1 from 2 to 5\nmove 12 from 9 to 7\nmove 4 from 4 to 9\nmove 4 from 9 to 5\nmove 13 from 5 to 4\nmove 4 from 4 to 7\nmove 1 from 7 to 9\nmove 2 from 9 to 5\nmove 9 from 1 to 2\nmove 1 from 8 to 3\nmove 5 from 4 to 2\nmove 1 from 3 to 6\nmove 7 from 2 to 8\nmove 6 from 1 to 6\nmove 6 from 8 to 7\nmove 6 from 2 to 1\nmove 3 from 9 to 3\nmove 7 from 3 to 7\nmove 4 from 4 to 9\nmove 1 from 8 to 9\nmove 1 from 3 to 9\nmove 1 from 2 to 4\nmove 1 from 9 to 6\nmove 5 from 1 to 9\nmove 1 from 4 to 9\nmove 2 from 9 to 1\nmove 8 from 6 to 7\nmove 4 from 9 to 7\nmove 2 from 5 to 2\nmove 2 from 1 to 9\nmove 14 from 7 to 4\nmove 22 from 7 to 2\nmove 2 from 7 to 4\nmove 3 from 7 to 5\nmove 9 from 4 to 7\nmove 6 from 2 to 4\nmove 8 from 4 to 3\nmove 14 from 2 to 9\nmove 2 from 3 to 9\nmove 3 from 2 to 9\nmove 4 from 4 to 2\nmove 1 from 4 to 5\nmove 1 from 1 to 4\nmove 5 from 7 to 8\nmove 1 from 1 to 3\nmove 4 from 5 to 2\nmove 6 from 3 to 9\nmove 1 from 3 to 4\nmove 4 from 8 to 9\nmove 2 from 4 to 6\nmove 4 from 5 to 3\nmove 1 from 7 to 6\nmove 1 from 8 to 5\nmove 3 from 3 to 1\nmove 33 from 9 to 5\nmove 5 from 2 to 1\nmove 1 from 3 to 5\nmove 1 from 7 to 6\nmove 18 from 5 to 1\nmove 1 from 2 to 8\nmove 6 from 5 to 4\nmove 1 from 8 to 7\nmove 2 from 4 to 1\nmove 4 from 1 to 2\nmove 19 from 1 to 2\nmove 4 from 6 to 8\nmove 4 from 1 to 8\nmove 14 from 2 to 9\nmove 5 from 2 to 4\nmove 1 from 8 to 2\nmove 8 from 2 to 5\nmove 5 from 8 to 4\nmove 4 from 9 to 7\nmove 1 from 8 to 1\nmove 16 from 5 to 4\nmove 15 from 4 to 5\nmove 1 from 9 to 5\nmove 5 from 7 to 6\nmove 2 from 7 to 6\nmove 1 from 1 to 9\nmove 7 from 6 to 7\nmove 1 from 8 to 5\nmove 1 from 1 to 9\nmove 12 from 5 to 7\nmove 7 from 5 to 9\nmove 12 from 7 to 2\nmove 1 from 7 to 4\nmove 7 from 4 to 7\nmove 2 from 9 to 4\nmove 5 from 4 to 9\nmove 8 from 2 to 3\nmove 4 from 2 to 4\nmove 9 from 4 to 8\nmove 6 from 3 to 5\nmove 8 from 7 to 3\nmove 1 from 4 to 3\nmove 7 from 8 to 9\nmove 4 from 5 to 4\nmove 6 from 3 to 1\nmove 4 from 3 to 4\nmove 1 from 3 to 6\nmove 6 from 4 to 9\nmove 1 from 6 to 5\nmove 17 from 9 to 4\nmove 3 from 7 to 3\nmove 1 from 7 to 9\nmove 2 from 5 to 3\nmove 2 from 1 to 3\nmove 2 from 8 to 9\nmove 1 from 5 to 1\nmove 14 from 4 to 5\nmove 2 from 3 to 2\nmove 1 from 7 to 6\nmove 10 from 9 to 4\nmove 12 from 9 to 4\nmove 9 from 4 to 5\nmove 1 from 2 to 9\nmove 13 from 5 to 9\nmove 2 from 5 to 1\nmove 1 from 2 to 9\nmove 3 from 4 to 2\nmove 12 from 4 to 7\nmove 8 from 5 to 7\nmove 1 from 1 to 9\nmove 1 from 6 to 4\nmove 1 from 5 to 4\nmove 1 from 4 to 8\nmove 5 from 3 to 4\nmove 10 from 9 to 6\nmove 3 from 6 to 2\nmove 7 from 6 to 5\nmove 6 from 5 to 4\nmove 1 from 8 to 5\nmove 1 from 1 to 4\nmove 2 from 7 to 2\nmove 5 from 4 to 9\nmove 2 from 5 to 8\nmove 1 from 1 to 3\nmove 2 from 1 to 7\nmove 6 from 7 to 9\nmove 9 from 9 to 8\nmove 1 from 1 to 3\nmove 4 from 2 to 7\nmove 11 from 7 to 3\nmove 11 from 8 to 6\nmove 7 from 3 to 1\nmove 4 from 7 to 2\nmove 3 from 2 to 9\nmove 8 from 1 to 5\nmove 2 from 7 to 5\nmove 2 from 2 to 9\nmove 2 from 3 to 9\nmove 11 from 4 to 7\nmove 7 from 9 to 5\nmove 6 from 6 to 5\nmove 2 from 2 to 9\nmove 1 from 2 to 3\nmove 6 from 9 to 4\nmove 3 from 9 to 1\nmove 4 from 3 to 5\nmove 6 from 7 to 1\nmove 2 from 6 to 3\nmove 2 from 9 to 2\nmove 3 from 3 to 2\nmove 3 from 6 to 8\nmove 2 from 7 to 5\nmove 20 from 5 to 6\nmove 8 from 5 to 1\nmove 1 from 5 to 9\nmove 2 from 8 to 4\nmove 1 from 8 to 7\nmove 16 from 1 to 8\nmove 8 from 8 to 9\nmove 4 from 2 to 4\nmove 1 from 1 to 5\nmove 1 from 5 to 4\nmove 3 from 8 to 4\nmove 14 from 4 to 6\nmove 5 from 8 to 7\nmove 6 from 7 to 8\nmove 29 from 6 to 2\nmove 3 from 9 to 8\nmove 21 from 2 to 3\nmove 1 from 8 to 3\nmove 6 from 9 to 4\nmove 8 from 3 to 5\nmove 7 from 8 to 4\nmove 7 from 3 to 9\nmove 3 from 7 to 2\nmove 12 from 4 to 8\nmove 2 from 3 to 1\nmove 2 from 9 to 1\nmove 1 from 6 to 7\nmove 1 from 7 to 6\nmove 1 from 6 to 3\nmove 3 from 1 to 8\nmove 2 from 4 to 1\nmove 4 from 6 to 1\nmove 5 from 2 to 7\nmove 1 from 1 to 2\nmove 5 from 1 to 2\nmove 2 from 8 to 1\nmove 1 from 4 to 5\nmove 9 from 8 to 4\nmove 3 from 7 to 9\nmove 7 from 5 to 7\nmove 2 from 5 to 9\nmove 4 from 9 to 2\nmove 3 from 3 to 2\nmove 5 from 2 to 7\nmove 2 from 8 to 2\nmove 2 from 7 to 3\nmove 1 from 8 to 6\nmove 2 from 1 to 2\nmove 1 from 6 to 7\nmove 1 from 8 to 1\nmove 12 from 7 to 1\nmove 5 from 2 to 7\nmove 7 from 4 to 2\nmove 2 from 4 to 1\nmove 5 from 3 to 8\nmove 7 from 1 to 9\nmove 4 from 7 to 1\nmove 7 from 1 to 5\nmove 12 from 9 to 2\nmove 27 from 2 to 4\nmove 3 from 8 to 9\nmove 6 from 2 to 5\nmove 6 from 1 to 8\nmove 1 from 7 to 6\nmove 9 from 5 to 2\nmove 3 from 9 to 2\nmove 13 from 4 to 5\nmove 10 from 2 to 7\nmove 1 from 9 to 8\nmove 11 from 5 to 7\nmove 1 from 8 to 7\nmove 1 from 2 to 6\nmove 13 from 4 to 3\nmove 23 from 7 to 4\nmove 1 from 6 to 9\nmove 1 from 2 to 4\nmove 7 from 3 to 5\nmove 1 from 9 to 8\nmove 19 from 4 to 1\nmove 2 from 4 to 1\nmove 1 from 7 to 6\nmove 1 from 4 to 5\nmove 1 from 5 to 7\nmove 11 from 5 to 1\nmove 2 from 5 to 4\nmove 2 from 6 to 9\nmove 3 from 8 to 2\nmove 2 from 8 to 1\nmove 3 from 2 to 1\nmove 1 from 9 to 5\nmove 6 from 1 to 3\nmove 1 from 9 to 7\nmove 2 from 7 to 5\nmove 2 from 8 to 6\nmove 1 from 3 to 2\nmove 2 from 8 to 5\nmove 1 from 2 to 1\nmove 3 from 4 to 1\nmove 3 from 5 to 1\nmove 2 from 5 to 1\nmove 2 from 6 to 9\nmove 1 from 9 to 6\nmove 1 from 4 to 5\nmove 1 from 9 to 8\nmove 1 from 8 to 6\nmove 8 from 1 to 6\nmove 7 from 1 to 8\nmove 9 from 1 to 6\nmove 1 from 5 to 3\nmove 3 from 8 to 4\nmove 11 from 3 to 4\nmove 1 from 3 to 6\nmove 10 from 6 to 8\nmove 13 from 1 to 6\nmove 3 from 4 to 5\nmove 7 from 8 to 6\nmove 3 from 8 to 5\nmove 6 from 5 to 3\nmove 22 from 6 to 9\nmove 4 from 3 to 6\nmove 4 from 9 to 5\nmove 1 from 1 to 5\nmove 2 from 3 to 4\nmove 2 from 1 to 5\nmove 1 from 9 to 2\nmove 5 from 8 to 3\nmove 2 from 9 to 2\nmove 11 from 6 to 9\nmove 3 from 2 to 7\nmove 1 from 6 to 7\nmove 12 from 9 to 8\nmove 4 from 7 to 1\nmove 12 from 4 to 8\nmove 2 from 4 to 7\nmove 1 from 1 to 8\nmove 1 from 5 to 1\nmove 19 from 8 to 4\nmove 4 from 5 to 1\nmove 1 from 7 to 4\nmove 1 from 7 to 1\nmove 3 from 3 to 4\nmove 2 from 8 to 4\nmove 1 from 5 to 7\nmove 1 from 7 to 9\nmove 8 from 1 to 8\nmove 1 from 1 to 4\nmove 1 from 3 to 9\nmove 1 from 3 to 5\nmove 1 from 5 to 2\nmove 7 from 8 to 7\nmove 16 from 4 to 7\nmove 1 from 7 to 4\nmove 3 from 8 to 2\nmove 14 from 7 to 4\nmove 1 from 5 to 8\nmove 5 from 7 to 5\nmove 16 from 4 to 5\nmove 3 from 5 to 4\nmove 3 from 2 to 1\nmove 1 from 7 to 9\nmove 11 from 4 to 2\nmove 3 from 8 to 6\nmove 2 from 1 to 8\nmove 1 from 4 to 9\nmove 18 from 5 to 1\nmove 1 from 8 to 7\nmove 3 from 7 to 9\nmove 18 from 9 to 3\nmove 3 from 6 to 9\nmove 7 from 1 to 6\nmove 1 from 8 to 4\nmove 1 from 4 to 9\nmove 3 from 6 to 4\nmove 5 from 9 to 2\nmove 2 from 4 to 7\nmove 7 from 2 to 8\nmove 1 from 7 to 3\nmove 2 from 6 to 8\nmove 1 from 9 to 5\nmove 1 from 6 to 8\nmove 1 from 4 to 8\nmove 1 from 5 to 3\nmove 1 from 7 to 5\nmove 8 from 8 to 7\nmove 10 from 2 to 6\nmove 1 from 9 to 3\nmove 6 from 6 to 2\nmove 5 from 6 to 2\nmove 7 from 2 to 7\nmove 12 from 1 to 6\nmove 2 from 2 to 1\nmove 1 from 2 to 5\nmove 4 from 7 to 6\nmove 12 from 3 to 1\nmove 2 from 7 to 2\nmove 9 from 3 to 8\nmove 1 from 2 to 6\nmove 1 from 5 to 4\nmove 9 from 6 to 5\nmove 1 from 7 to 6\nmove 1 from 4 to 9\nmove 9 from 6 to 7\nmove 7 from 8 to 3\nmove 6 from 3 to 1\nmove 4 from 8 to 3\nmove 5 from 3 to 1\nmove 1 from 9 to 8\nmove 2 from 8 to 9\nmove 5 from 5 to 7\nmove 14 from 7 to 8\nmove 1 from 9 to 4\nmove 2 from 2 to 1\nmove 3 from 5 to 3\nmove 2 from 3 to 1\nmove 1 from 4 to 6\nmove 6 from 8 to 6\nmove 6 from 8 to 3\nmove 3 from 6 to 1\nmove 2 from 8 to 9\nmove 19 from 1 to 6\nmove 3 from 9 to 3\nmove 6 from 3 to 4\nmove 6 from 6 to 2\nmove 4 from 3 to 9\nmove 1 from 7 to 9\nmove 2 from 5 to 7\nmove 5 from 9 to 6\nmove 6 from 7 to 2\nmove 11 from 2 to 5\nmove 2 from 7 to 4\nmove 4 from 4 to 3\nmove 2 from 4 to 8\nmove 12 from 1 to 2\nmove 1 from 8 to 2\nmove 8 from 5 to 7\nmove 2 from 4 to 9\nmove 2 from 7 to 1\nmove 4 from 2 to 3\nmove 1 from 8 to 6\nmove 1 from 1 to 5\nmove 2 from 9 to 1\nmove 2 from 7 to 3\nmove 2 from 5 to 2\nmove 1 from 5 to 7\nmove 2 from 7 to 8\nmove 1 from 5 to 7\nmove 5 from 3 to 4\nmove 3 from 1 to 7\nmove 1 from 2 to 4\nmove 15 from 6 to 1\nmove 4 from 4 to 1\nmove 4 from 2 to 3\nmove 8 from 3 to 2\nmove 5 from 2 to 4\nmove 1 from 8 to 6\nmove 1 from 8 to 9\nmove 1 from 3 to 1\nmove 3 from 7 to 3\nmove 5 from 7 to 6\nmove 4 from 2 to 9\nmove 6 from 2 to 6\nmove 4 from 9 to 6\nmove 12 from 1 to 5\nmove 6 from 4 to 1\nmove 1 from 3 to 6\nmove 4 from 5 to 8\nmove 7 from 5 to 3\nmove 3 from 8 to 2\nmove 1 from 2 to 3\nmove 1 from 9 to 5\nmove 1 from 4 to 5\nmove 1 from 8 to 5\nmove 8 from 6 to 9\nmove 10 from 1 to 4\nmove 3 from 6 to 1\nmove 9 from 3 to 6\nmove 1 from 3 to 8\nmove 1 from 2 to 4\nmove 6 from 9 to 1\nmove 1 from 1 to 4\nmove 10 from 1 to 6\nmove 1 from 8 to 6\nmove 13 from 6 to 7\nmove 1 from 2 to 1\nmove 1 from 9 to 6\nmove 9 from 7 to 5\nmove 1 from 9 to 4\nmove 3 from 7 to 1\nmove 3 from 5 to 6\nmove 10 from 4 to 7\nmove 5 from 6 to 5\nmove 3 from 4 to 5\nmove 13 from 6 to 9\nmove 7 from 5 to 3\nmove 6 from 3 to 2\nmove 5 from 6 to 4\nmove 4 from 2 to 8"));
}

/*
--- Day 5: Supply Stacks ---

The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3

In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3

Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3

Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3

The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?
*/
